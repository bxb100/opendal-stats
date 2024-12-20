---
theme: dashboard
---

# Dashboard

```js
import {CodeAgePlot} from "./components/codeAgePlot.js";
import {CommitCalendarPlot} from "./components/commitCalendarPlot.js";
import {CommitRankPlot} from "./components/commitRankPlot.js";
import {revive} from "./components/revive.js";
import {Trend} from "./components/trend.js";
import {BurndownPlot} from "./components/burndownPlot.js";
import {DailyPlot} from "./components/dailyPlot.js";
import {parse} from "./components/parsecodeage.js";
```

```js
const issues = FileAttachment("data/github-issues.json").json().then(revive);
const stars = FileAttachment("data/github-stars.csv").csv({ typed: true });
const codeage = FileAttachment("data/codeage.csv").dsv({ array: true }).then(parse);
const commits = FileAttachment("data/github-commit.json").json().then(revive);
```

```js
// These dates are declared globally to ensure consistency across plots.
const end = new Date();
const today = new Date();
const start = d3.utcYear.offset(end, -2);
const lastMonth = d3.utcDay.offset(end, -28);
const lastWeek = d3.utcDay.offset(end, -7);
const x = {domain: [start, end]};
const lastYear = d3.utcYear.offset(end, -1);
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
  <div class="card">
    ${CommitCalendarPlot(commits, {lastYear, today, dark})}
  </div>
</div>

<div class="grid grid-cols-1">
  <div class="card">
    ${CommitRankPlot(commits, {today})}
  </div>
</div>

<div class="grid">
  <div class="card">${CodeAgePlot(codeage)}</div>
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
