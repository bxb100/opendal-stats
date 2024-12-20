#!/usr/bin/env rust-script
//!
//! ```cargo
//! [dependencies]
//! rayon = "1.10.0"
//! chrono = "0.4.39"
//! anyhow = "1.0.94"
//! ```

use anyhow::Result;
use chrono::prelude::*;
use chrono::Months;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::OnceLock;

type Bucket = BTreeMap<&'static DateTime<Utc>, u32>;

fn headers() -> &'static String {
    static CSV_HEADERS: OnceLock<String> = OnceLock::new();
    CSV_HEADERS.get_or_init(|| {
        let segment = get_date_segment();
        let mut arr = vec![
            "date".to_owned(),
            format!("< {}", segment[0].format("%Y-%m-%d")),
        ];
        for seg in segment.iter().skip(1) {
            arr.push(format!("≥ {}", seg.format("%Y-%m-%d")));
        }
        arr.join(",")
    })
}

fn get_date_segment() -> &'static Vec<DateTime<Utc>> {
    static SEGMENT: OnceLock<Vec<DateTime<Utc>>> = OnceLock::new();
    SEGMENT.get_or_init(|| {
        let mut start_date = Utc.with_ymd_and_hms(2022, 3, 1, 0, 0, 0).unwrap();
        let end_date = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let mut dates: Vec<DateTime<Utc>> = Vec::new();

        while start_date <= end_date {
            dates.push(start_date);
            start_date = start_date.checked_add_months(Months::new(3)).unwrap();
        }
        dates
    })
}

fn main() -> Result<()> {
    let data = show()?;

    let mut stdout = std::io::stdout();
    writeln!(&mut stdout, "{}", headers())?;
    write!(&mut stdout, "{data}")?;

    Ok(())
}

fn command(args: Vec<&str>) -> Result<String> {
    let path = env::var("REPO_PATH").expect("Need to set env REPO_PATH");

    let output = Command::new("git")
        .args(args)
        .current_dir(path)
        .stdout(Stdio::piped())
        .output()?;

    let res = String::from_utf8(output.stdout)?;
    Ok(res)
}

fn single_file(tag: &str, file: &str) -> Result<Vec<i64>> {
    // https://git-scm.com/docs/git-blame
    let content = command(vec!["blame", "-t", "--line-porcelain", file, tag])?;

    let res = content
        .lines()
        .par_bridge()
        .filter(|l| l.starts_with("committer-time"))
        .filter_map(|l| l[15..].parse::<i64>().ok())
        .collect::<Vec<_>>();

    Ok(res)
}

#[test]
fn test_single_file() {
    let res = single_file("v0.26.0", "src/services/azblob/backend.rs").unwrap();
    println!("{:?}", res)
}

macro_rules! filter_many {
    ($line:ident, $ext:tt, $($other:tt),* ) => {
        !$line.ends_with($ext) $(&& !$line.ends_with($other))*
    };
}

fn tag_files(tag: &str) -> Result<Vec<String>> {
    // https://git-scm.com/docs/git-ls-tree
    let files = command(vec!["ls-tree", "-r", "--name-only", tag])?;

    let res = files.lines()
        .filter(|l| !l.starts_with(".") && !l.starts_with("website"))
        .filter(|l| filter_many!(l, ".png", ".yaml", ".yml", ".md", ".jpg", ".lock"))
        .map(|l| l.to_owned()).collect::<Vec<_>>();

    Ok(res)
}

#[test]
fn test_tag_files() {
    let res = tag_files("v0.26.0").unwrap();
    println!("{:?}", res);
}

fn merge_tag_files_commit_times(tag: &str) -> Result<Vec<i64>> {
    let files = tag_files(tag)?;
    let timestamps = files
        .par_iter()
        .filter_map(|file| single_file(tag, file).ok())
        .flatten()
        .collect::<Vec<_>>();

    Ok(timestamps)
}

#[test]
fn test_merge_tag_files_commit_times() {
    let s = merge_tag_files_commit_times("v0.26.0").unwrap();
    println!("{:?}", s)
}

fn merge(tag: &str) -> Result<Bucket> {
    let ts = merge_tag_files_commit_times(tag)?;
    let segments = get_date_segment();

    let mut map: Bucket = BTreeMap::new();

    for x in ts {
        for date in segments {
            let v = map.entry(date).or_insert(0);
            if date.timestamp() >= x {
                *v += 1;
            }
        }
    }

    Ok(map)
}

#[test]
fn test_merge() {
    let data = merge("v0.26.0").unwrap();
    println!("{:?}", data);
}

fn get_tag_create_at(tag: &str) -> Option<DateTime<Utc>> {
    // https://git-scm.com/docs/git-show
    let content = command(vec![
        "show",
        "--format=%at",
        "-s",
        &*format!("{}^{{commit}}", tag),
    ])
    .ok()?;
    let timestamp = content.trim().parse::<i64>().ok()?;
    DateTime::from_timestamp(timestamp, 0)
}

#[test]
fn test_get_tag_create_at() {
    let data = get_tag_create_at("v0.26.0").unwrap();
    println!("{:?}", data);
}

fn get_tags() -> Result<Vec<String>> {
    let content = command(vec!["tag", "-l"])?;

    let res = content
        .lines()
        .filter(|l| l.starts_with("v") && !l.contains("rc"))
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    Ok(res)
}

#[test]
fn test_get_tags() {
    let data = get_tags().unwrap();
    println!("{:?}", data);
}

fn show() -> Result<String> {
    let tags = get_tags()?;

    let mut data = tags
        .par_iter()
        .map(|tag| {
            let create_at = get_tag_create_at(tag).unwrap();
            let data = merge(tag).unwrap();
            let s = data
                .values()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(",");
            (create_at, s)
        })
        .collect::<Vec<_>>();

    data.sort_by(|a, b| a.0.cmp(&b.0));

    let res = data
        .into_iter()
        .map(|(date, s)| format!("{},{}", date.format("%Y-%m-%d"), s))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(res)
}

#[test]
fn test_show() {
    let data = show().unwrap();
    println!("{:?}", data)
}
