import * as Plot from "../../_npm/@observablehq/plot@0.6.16/3ca91556.js";
import { resize } from "../../_observablehq/stdlib.95bfbf7e.js";
import * as d3 from "../../_npm/d3@7.9.0/93fe06f7.js";
import { greatest } from "../../_npm/d3-array@3.2.4/e95f898e.js";

export function CommitRankPlot(commits, { today }) {
  const commitsStart = greatest([ d3.utcYear.offset(today, -3), d3.utcDay(commits.at(-1).date) ]);
  const commitDomain = [ new Date(commitsStart), today ];

  return resize((width) => Plot.plot({
    title: "Commits by author",
    subtitle: new Set(commits.filter((d) => d.date >= commitsStart).map((d) => d.author)).size > 10 ? "Top 10 authors" : "",
    width,
    label: null,
    marginLeft: 0,
    marginRight: 60,
    marginBottom: 0,
    x: { axis: "top", commitDomain },
    y: { grid: true },
    marks: [
      Plot.axisY({ anchor: "right", textOverflow: "ellipsis-middle", lineWidth: 5 }),
      Plot.dot(commits.filter((d) => d.date >= commitsStart), {
        x: "date",
        y: "author",
        sort: { y: "x", reduce: "count", reverse: true, limit: 10 }
      }),
      Plot.voronoi(commits.filter((d) => d.date >= commitsStart), {
        x: "date",
        y: "author",
        href: (d) => `https://github.com/apache/opendal/commit/${ d.sha }`,
        target: "_blank",
        fill: "transparent",
        title: "message",
        tip: { maxRadius: Infinity }
      })
    ]
  }))
}
