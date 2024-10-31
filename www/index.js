import { convert } from "jcampconverter";
import * as wasm from "conrec-wasm";
import { dirname } from "path";
import { fileURLToPath } from "url";
import { readFileSync, writeFileSync } from "fs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Read WASM file
wasm.initSync(readFileSync(`${__dirname}/../pkg/conrec_wasm_bg.wasm`));

const data = readFileSync(`${__dirname}/zhmbc_0.jdx`, "utf8");
const parsed = convert(data, { noContour: true }).flatten[0];
const matrix = parsed.minMax?.z || [];
const results = {};

const conrec = new wasm.ConrecWasm(matrix, {});

try {
  try {
    const result = conrec.draw_contour({
      contourDrawer: "basic",
      levels: [-1000000000, 1000000000],
      timeout: 10000,
    });
    results.test1 = result.Basic.contours;

    const result2 = conrec.draw_contour({
      contourDrawer: "basic",
      levels: [-100000, 100000],
      timeout: 10000,
    });
    results.test2 = result2.Basic.contours;

    const result3 = conrec.draw_contour({
      contourDrawer: "basic",
      levels: [],
      timeout: 10000,
    });
    results.test3 = result3.Basic.contours;

    const result4 = conrec.draw_contour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10000,
    });
    results.test4 = result4.Basic.contours;

    let matrixSwap = new wasm.ConrecWasm(matrix, { swap_axes: true });
    const result5 = matrixSwap.draw_contour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10000,
    });
    results.test5 = result5.Basic.contours;

    const result6 = conrec.draw_contour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10,
    });
    results.test6 = result6.Basic.contours;

    writeFileSync(
      `${__dirname}/contours.json`,
      JSON.stringify(results, null, 2)
    );

    console.log("Success", result.Basic.contours);
  } catch (err) {
    console.error("Error", err);
  }
} catch (error) {
  console.error("Main error:", error);
  console.log("\nDebug info:");
  console.log(
    "ConrecWasm methods:",
    Object.getOwnPropertyNames(wasm.ConrecWasm.prototype)
  );
  console.log("Available exports:", Object.keys(wasm));
}

try {
  conrec.free();
} catch (e) {
  console.log("Cleanup error:", e);
}
