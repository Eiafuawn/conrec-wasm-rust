import init, { ConrecWasm } from "../pkg/conrec_wasm.js";
import { readFileSync, writeFileSync } from "fs";
import { convert } from "jcampconverter";
await init();

const data = readFileSync(`${__dirname}/zhmbc_0.jdx`, "utf8");
const parsed = convert(data, { noContour: true }).flatten[0];
const matrix = parsed.minMax?.z || [];
writeFileSync(`${__dirname}/matrix.json`, JSON.stringify(matrix, null, 2));

const conrec = new ConrecWasm(matrix, {
  swap_axes: false,
  contour_drawer: "Basic",
  levels: [-1000000000, 1000000000],
  timeout: 10000,
});
