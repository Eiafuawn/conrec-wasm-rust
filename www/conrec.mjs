import { readFileSync } from "fs";
import { dirname } from "path";
import { fileURLToPath } from "url";
import { convert } from "jcampconverter";
import { Conrec } from "ml-conrec";
import * as wasm from "conrec-wasm";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const data = readFileSync(`${__dirname}/zhmbc_0.jdx`, "utf8");
const parsed = convert(data, { noContour: true }).flatten[0];
const matrix = parsed.minMax?.z || [];

console.log(`Matrix size: ${matrix[0].length} x ${matrix.length}`);

function runJSTests() {
  console.log("\n=== JavaScript (ml-conrec) Tests ===");

  try {
    console.time("JS Test 1");
    const conrec = new Conrec(matrix, {});
    for (let i = 0; i < 100; i++) {
      const result = conrec.drawContour({
        contourDrawer: "basic",
        levels: [-1000000000, 1000000000],
        timeout: 10000,
      });
      if (i === 99) {
        console.log("JS Test 1 lines:", result.contours[0]?.lines.length || 0);
      }
    }
    console.timeEnd("JS Test 1");

    console.time("JS Test 2");
    const conrec2 = new Conrec(matrix, {});
    for (let i = 0; i < 100; i++) {
      const result2 = conrec2.drawContour({
        contourDrawer: "basic",
        levels: [-100000, 100000],
        timeout: 10000,
      });
      if (i === 99) {
        console.log("JS Test 2 lines:", result2.contours[0]?.lines.length || 0);
      }
    }
    console.timeEnd("JS Test 2");

    console.time("JS Test 4");
    const conrec4 = new Conrec(matrix, {});
    for (let i = 0; i < 20; i++) {
      const result4 = conrec4.drawContour({
        contourDrawer: "basic",
        nbLevels: 10,
        timeout: 10000,
      });
      if (i === 19) {
        console.log("JS Test 4 lines:", result4.contours[1]?.lines.length || 0);
      }
    }
    console.timeEnd("JS Test 4");
  } catch (err) {
    console.error("JS Error:", err);
  }
}

function runWASMTests() {
  console.log("\n=== WASM Tests ===");

  try {
    console.time("WASM Test 1");
    const conrec = new wasm.ConrecWasm(matrix, {});
    for (let i = 0; i < 100; i++) {
      const result = conrec.draw_contour({
        contourDrawer: "basic",
        levels: [-1000000000, 1000000000],
        timeout: 10000,
      });
      // if (i === 99) {
      //   console.log(
      //     "WASM Test 1 lines:",
      //     result.contours[0]?.lines.length || 0
      //   );
      // }
    }
    console.timeEnd("WASM Test 1");

    console.time("WASM Test 2");
    const conrec2 = new wasm.ConrecWasm(matrix, {});
    for (let i = 0; i < 100; i++) {
      const result2 = conrec2.draw_contour({
        contourDrawer: "basic",
        levels: [-100000, 100000],
        timeout: 10000,
      });
      // if (i === 99) {
      //   console.log(
      //     "WASM Test 2 lines:",
      //     result2.contours[0]?.lines.length || 0
      //   );
      // }
    }
    console.timeEnd("WASM Test 2");

    console.time("WASM Test 4");
    const conrec4 = new wasm.ConrecWasm(matrix, {});
    for (let i = 0; i < 20; i++) {
      const result4 = conrec4.draw_contour({
        contourDrawer: "basic",
        nbLevels: 10,
        timeout: 10000,
      });
      // if (i === 19) {
      //   console.log(
      //     "WASM Test 4 lines:",
      //     result4.contours[1]?.lines.length || 0
      //   );
      // }
    }
    console.timeEnd("WASM Test 4");
  } catch (err) {
    console.error("WASM Error:", err);
  }
}

// Run both test suites
console.log("Starting benchmark comparison...");
runJSTests();
runWASMTests();

console.log("\n=== Performance Comparison Complete ===");
