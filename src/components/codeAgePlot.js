import * as Plot from "npm:@observablehq/plot";
import { resize } from "npm:@observablehq/stdlib";

export function CodeAgePlot(data) {

  return resize(width => {
    return Plot.plot({
      marginLeft: 20,
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
        Plot.areaY(data, {x: "date", y: "value", y2: 0, fill: "name", title: "name"}),
        Plot.ruleY([0]),
      ]
    })
  })
}
