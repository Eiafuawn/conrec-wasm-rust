import { convert } from "jcampconverter";
import * as wasm from "conrec-wasm";
import { dirname } from "path";
import { fileURLToPath } from "url";
import { readFileSync, write, writeFileSync } from "fs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Read WASM file
wasm.initSync(readFileSync(`${__dirname}/../pkg/conrec_wasm_bg.wasm`));

const data = readFileSync(`${__dirname}/zhmbc_0.jdx`, "utf8");
const parsed = convert(data, { noContour: true }).flatten[0];
const matrix = parsed.minMax?.z || [];
writeFileSync(`${__dirname}/matrix.json`, JSON.stringify(matrix, null, 2));

const conrec = new wasm.ConrecWasm(matrix, {});

try {
  try {
    console.time("Test 1");
    for (let i = 0; i < 100; i++) {
      const result = conrec.draw_contour({
        contour_drawer: "Basic",
        levels: [-1000000000, 1000000000],
        timeout: 10000,
      });
    }
    console.timeEnd("Test 1");

    console.time("Test 2");
    for (let i = 0; i < 100; i++) {
      const result2 = conrec.draw_contour({
        contour_drawer: "Basic",
        levels: [-100000, 100000],
        timeout: 10000,
      });
    }
    console.timeEnd("Test 2");

    console.time("Test 3");
    for (let i = 0; i < 500; i++) {
      const result3 = conrec.draw_contour({
        contour_drawer: "Basic",
        levels: [],
        timeout: 10000,
      });
    }
    console.timeEnd("Test 3");

    console.time("Test 4");
    for (let i = 0; i < 20; i++) {
      const result4 = conrec.draw_contour({
        contour_drawer: "Basic",
        levels: [10],
        timeout: 10000,
      });
    }
    console.timeEnd("Test 4");

    let matrixSwap = new wasm.ConrecWasm(matrix, { swap_axes: true });
    console.time("Test 5");
    for (let i = 0; i < 20; i++) {
      const result5 = matrixSwap.draw_contour({
        contour_drawer: "Basic",
        levels: [10],
        timeout: 10000,
      });
    }
    console.timeEnd("Test 5");

    console.time("Test 6");
    for (let i = 0; i < 20; i++) {
      const result6 = conrec.draw_contour({
        contour_drawer: "Basic",
        levels: [10],
        timeout: 10,
      });
    }
    console.timeEnd("Test 6");
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
