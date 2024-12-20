---
theme: dashboard
---

# Dashboard

```js
import {revive} from "./components/revive.js";
import {Trend} from "./components/trend.js";
import {BurndownPlot} from "./components/burndownPlot.js";
import {DailyPlot} from "./components/dailyPlot.js";
import {utcParse} from "npm:d3-time-format";
```

```js
const issues = FileAttachment("data/github-issues.json").json().then(revive);
const stars = FileAttachment("data/github-stars.csv").csv({ typed: true });
const codeagedata = FileAttachment("data/codeage.csv").dsv({ array: true });
```

```js
// These dates are declared globally to ensure consistency across plots.
const end = new Date();
const start = d3.utcYear.offset(end, -2);
const lastMonth = d3.utcDay.offset(end, -28);
const lastWeek = d3.utcDay.offset(end, -7);
const x = {domain: [start, end]};
const parseDate = utcParse("%Y-%m-%d");
```

```js
let jsondata = []

let header = codeagedata[0];
let data = codeagedata.slice(1);
let len = header.length;

for (let j = 0;j < data.length; j++) {
    const covert = (index) => {
        return {
            date: parseDate(data[j][0]),
            value: Number.parseInt(data[j][index]),
            name: header[index]
        }
    };
    for (let i = len - 1; i > 1; i--) {
        jsondata.push(covert(i));
    }
    jsondata.push(covert(1));
}

function codeage(width) {
    return Plot.plot({
        marginLeft: 20,
        //
        title: "Source code age",
        subtitle: "Lines of code written per three-month segment",
        width: width,
        y: {
            grid: true,
            axis: "right",
            label: "Lines of code (including blanks and comments)",
            tickFormat: (d) => d / 1000 + "K",
        },
        color: {legend: true,columns: 1, swatchWidth: 50, style: "position: absolute;left: 20px;margin-top: 40px;"},
        marks: [
            Plot.areaY(jsondata, {x: "date", y: "value", y2: 0, fill: "name", title: "name"}),
            Plot.ruleY([0]),
        ]
    })
}
```

<div class="grid" style="grid-template-columns: repeat(5, minmax(0, 1fr));">
  <a class="card" href="https://github.com/apache/opendal/issues?q=is:issue+is:open+sort:updated-desc" style="color: inherit;">
    <h2>Open issues</h2>
    <span class="big">${d3.sum(issues, (d) => !d.pull_request && d.state === "open").toLocaleString("en-US")}</span>
  </a>
  <a class="card" href="https://github.com/apache/opendal/pulls?q=is:pr+is:open+sort:updated-desc+draft:false" style="color: inherit;">
    <h2>Open PRs</h2>
    <span class="big">${d3.sum(issues, (d) => d.pull_request && d.state === "open" && !d.draft).toLocaleString("en-US")}</span>
  </a>
  <a class="card" href="https://github.com/apache/opendal/issues?q=sort:created-desc" style="color: inherit;">
    <h2>Recent opened issues</h2>
    <span class="big">${d3.sum(issues, (d) => !d.pull_request && d.created_at >= lastMonth).toLocaleString("en-US")}</span>
    <span class="muted">in 28d</span>
  </a>
  <a class="card" href="https://github.com/apache/opendal/issues?q=is:issue+is:closed+sort:updated-desc" style="color: inherit;">
    <h2>Recent closed issues</h2>
    <span class="big">${d3.sum(issues, (d) => !d.pull_request && d.closed_at >= lastMonth).toLocaleString("en-US")}</span>
    <span class="muted">in 28d</span>
  </a>
  <a class="card" href="https://github.com/apache/opendal" style="color: inherit;">
    <h2>GitHub stars</h2>
    <span class="big">${stars.length.toLocaleString("en-US")}</span>
    ${Trend(d3.sum(stars, (d) => d.starred_at >= lastWeek))}
    <span class="muted">over 7d</span>
  </a>
</div>

<div class="grid grid-cols-1">
  <div class="card">${resize((width) => codeage(width))}</div>
</div>


<div class="grid">
  <div class="card">
    <h2>Open issues over time</h2>
    ${BurndownPlot(issues.filter((d) => !d.pull_request), {x, color: {legend: true, label: "open month"}})}
  </div>
</div>



<div class="grid">
  <div class="card" style="padding: 0">
    ${Inputs.table(
      issues
        .filter((d) => d.state === "open" && d.reactions.total_count > 5)
        .sort((a, b) => b.reactions.total_count - a.reactions.total_count)
        .map((d) => ({
          "title": {title: d.title, number: d.number},
          "reactions": d.reactions.total_count,
          "days old": d3.utcDay.count(d.created_at, end)
        })),
      {
        width,
        header: {
          title: "Top issues"
        },
        format: {
          title: (d) => html`<a href=https://github.com/apache/opendal/issues/${d.number} target=_blank>${d.title}</a>`
        }
      }
    )}
  </div>
</div>

<footer id="observablehq-footer" style="float: right;display: flex;flex-direction: column">
<div>Built at ${new Date().toDateString()}</div>
<div style="margin-top: 10px">With ❤️ by <a href="https://github.com/bxb100/opendal-stats">Lemon</a></div>
</footer>
