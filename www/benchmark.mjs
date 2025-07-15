import { run, bench, boxplot, do_not_optimize, group } from "mitata";

import * as wasm from "conrec-wasm";
import { Conrec } from "ml-conrec";
import { readFileSync } from "fs";
// const readFileSync = require("fs").readFileSync;
import { fileURLToPath } from "url";
// const fileURLToPath = require("url").fileURLToPath;
import { dirname } from "path";
// const dirname = require("path").dirname;
import { convert } from "jcampconverter";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const data = readFileSync(`${__dirname}/zhmbc_0.jdx`, "utf8");
const parsed = convert(data, { noContour: true }).flatten[0];
const matrix = parsed.minMax?.z || [];

function WasmOverheadConrec() {
  const conrec = new wasm.ConrecWasm(matrix, {});
  for (let i = 0; i < 100; i++) {
    const result = conrec.draw_contour({
      contourDrawer: "basic",
      levels: [-1000000000, 1000000000],
      timeout: 10000,
    });
  }

  for (let i = 0; i < 100; i++) {
    const result2 = conrec.draw_contour({
      contourDrawer: "basic",
      levels: [-100000, 100000],
      timeout: 10000,
    });
  }

  for (let i = 0; i < 500; i++) {
    const result3 = conrec.draw_contour({
      contourDrawer: "basic",
      levels: [],
      timeout: 10000,
    });
  }

  for (let i = 0; i < 20; i++) {
    const result4 = conrec.draw_contour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10000,
    });
  }

  let matrixSwap = new wasm.ConrecWasm(matrix, { swapAxes: true });
  for (let i = 0; i < 20; i++) {
    const result5 = matrixSwap.draw_contour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10000,
    });
  }

  for (let i = 0; i < 20; i++) {
    const result6 = conrec.draw_contour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10,
    });
  }
}
function JSconrec() {
  const conrec = new Conrec(matrix, {});
  for (let i = 0; i < 100; i++) {
    const result = conrec.drawContour({
      contourDrawer: "basic",
      levels: [-1000000000, 1000000000],
      timeout: 10000,
    });
  }

  for (let i = 0; i < 100; i++) {
    const result2 = conrec.drawContour({
      contourDrawer: "basic",
      levels: [-100000, 100000],
      timeout: 10000,
    });
  }

  for (let i = 0; i < 500; i++) {
    const result3 = conrec.drawContour({
      contourDrawer: "basic",
      levels: [],
      timeout: 10000,
    });
  }

  for (let i = 0; i < 20; i++) {
    const result4 = conrec.drawContour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10000,
    });
  }

  let matrixSwap = new Conrec(matrix, { swapAxes: true });
  for (let i = 0; i < 20; i++) {
    const result5 = matrixSwap.drawContour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10000,
    });
  }

  for (let i = 0; i < 20; i++) {
    const result6 = conrec.drawContour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10,
    });
  }
}

group("Conrec tests", () => {
  bench("JSconrec", () => {
    do_not_optimize(JSconrec());
  }).gc("inner");
  bench("Wasmconrec", () => {
    do_not_optimize(wasm.process_file());
  }).gc("inner");
  bench("WasmOverheadConrec", () => {
    do_not_optimize(WasmOverheadConrec());
  }).gc("inner");
});

await run();
