import { githubList } from "./github.js";

async function load(repo: string) {
  const commits: any[] = [];
  for await (const item of githubList(`/repos/${ repo }/commits`)) {
    commits.push({
      sha: item.sha,
      message: truncate(item.commit.message),
      date: new Date(item.commit.committer.date),
      author: item.author?.login
    });
  }
  return commits.reverse();
}

function truncate(message, length = 255) {
  message = message.replace(/\n\n.*/s, "");
  return message.length <= length ? message : `${ message.slice(0, length - 1) }…`;
}

process.stdout.write(JSON.stringify(await load("apache/opendal")));
