import * as wasm from "conrec-wasm";
import { dirname } from "path";
import { fileURLToPath } from "url";
import { readFileSync } from "fs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Read WASM file
wasm.initSync(readFileSync(`${__dirname}/../pkg/conrec_wasm_bg.wasm`));

// Test case with matrix, matching x and y coordinates
let matrix = [
  [0.0, 1.0, 2.0, 3.0],
  [1.0, 2.0, 3.0, 4.0],
  [2.0, 3.0, 4.0, 5.0],
  [3.0, 4.0, 5.0, 6.0],
];

// Create x coordinates for each column
const x = [0.0, 1.0, 2.0, 3.0];

// Create y coordinates for each row
const y = [0.0, 1.0, 2.0, 3.0];

// Create instance with complete options
const conrec = new wasm.ConrecWasm(matrix, {
  x: x, // Add x coordinates
  y: y, // Add y coordinates
  ilb: 0,
  iub: 2, // Use at least 2x2 area
  jlb: 0,
  jub: 2, // Use at least 2x2 area
});

// Single contour level
const levels = [2.5]; // Choose a value between the min and max of matrix

try {
  console.log("Testing with matrix:", matrix);
  console.log("X coordinates:", x);
  console.log("Y coordinates:", y);
  console.log("Contour levels:", levels);

  // Try both contour drawer types
  const drawerTypes = ["basic", "shape"];

  for (const drawerType of drawerTypes) {
    console.log(`\nTrying ${drawerType} drawer:`);
    try {
      const result = conrec.draw_contour({
        contourDrawer: drawerType,
        levels,
        timeout: 5000,
      });
      console.log(`Success with ${drawerType}:`, result);
      console.log(`Contour lines:`, result.Basic.contours);
    } catch (err) {
      console.log(`Failed with ${drawerType}:`, err);
    }
  }
} catch (error) {
  console.error("Main error:", error);

  // Debug info
  console.log("\nDebug info:");
  console.log(
    "ConrecWasm methods:",
    Object.getOwnPropertyNames(wasm.ConrecWasm.prototype)
  );
  console.log("Available exports:", Object.keys(wasm));
}

// Cleanup
try {
  conrec.free();
} catch (e) {
  console.log("Cleanup error:", e);
}
