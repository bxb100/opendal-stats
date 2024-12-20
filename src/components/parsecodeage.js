import {utcParse} from "npm:d3-time-format";

export function parse(codeagedata) {
  const parseDate = utcParse("%Y-%m-%d");

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

  return jsondata
}
