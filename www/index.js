import { convert } from "jcampconverter";
import * as wasm from "conrec-wasm";
import { dirname } from "path";
import { fileURLToPath } from "url";
import { readFileSync } from "fs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Read WASM file
wasm.initSync(readFileSync(`${__dirname}/../pkg/conrec_wasm_bg.wasm`));

const data = readFileSync(`${__dirname}/zhmbc_0.jdx`, "utf8");
const parsed = convert(data, { noContour: true }).flatten[0];
const matrix = parsed.minMax?.z || [];

const conrec = new wasm.ConrecWasm(matrix, {});

try {
  const drawerTypes = ["basic", "shape"];

  for (const drawerType of drawerTypes) {
    console.log(`\nTrying ${drawerType} drawer:`);
    try {
      const result = conrec.draw_contour({
        contourDrawer: drawerType,
        levels: [-100000, 100000],
        timeout: 10000,
      });
      console.log("Success", result.Basic.contours);
    } catch (err) {
      console.log(`Failed with ${drawerType}:`, err);
    }
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
