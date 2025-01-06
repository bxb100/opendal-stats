import * as Plot from "../../_npm/@observablehq/plot@0.6.16/8171784b.js";
import { resize } from "../../_observablehq/stdlib.95bfbf7e.js";
import * as d3 from "../../_npm/d3@7.9.0/7def3f80.js";

export function CommitCalendarPlot(commits, { lastYear, today, dark }) {

  return resize((width) => Plot.plot({
    title: "Commits calendar",
    subtitle: "Last 12 months",
    width,
    label: null,
    round: false,
    marginBottom: 0,
    aspectRatio: 1,
    padding: 0,
    x: { axis: null },
    y: {
      domain: [ -1, 1, 2, 3, 4, 5, 6, 0 ],
      ticks: [ 1, 2, 3, 4, 5, 6, 0 ],
      tickFormat: Plot.formatWeekday()
    },
    color: {
      type: "log",
      label: "commits",
      domain: [ 0.2, 20 ],
      interpolate: "hcl",
      range: dark ? [ d3.hcl(160, 40, 0), d3.hcl(140, 80, 80) ] : [ "white", d3.hcl(140, 70, 40) ]
    },
    marks: [
      Plot.cell(d3.utcDays(lastYear, today), {
        x: (d) => d3.utcMonday.count(0, d),
        y: (d) => d.getUTCDay(),
        stroke: "var(--theme-background)",
        r: 2,
        inset: 1.5
      }),
      Plot.text(d3.utcMondays(d3.utcMonday(lastYear), d3.utcMonday(today)).filter((d, i, D) => i === 0 || d.getUTCMonth() !== D[i - 1].getUTCMonth()), {
        x: (d) => d3.utcMonday.count(0, d),
        y: -1,
        text: d3.utcFormat("%b"),
        frameAnchor: "bottom-left"
      }),
      Plot.cell(commits.filter((d) => d.date >= lastYear), Plot.group({ fill: "count" }, {
        x: (d) => d3.utcMonday.count(0, d.date),
        y: (d) => d.date.getUTCDay(),
        channels: { date: ([ d ]) => d3.utcDay(d.date) },
        r: 2,
        tip: { format: { x: null, y: null } },
        inset: 1
      }))
    ]
  }))
}
