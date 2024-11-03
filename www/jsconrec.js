import { readFileSync, writeFileSync } from "fs";
import { dirname } from "path";
import { fileURLToPath } from "url";

import { convert } from "jcampconverter";

import { Conrec } from "ml-conrec";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const data = readFileSync(`${__dirname}/zhmbc_0.jdx`, "utf8");
const parsed = convert(data, { noContour: true }).flatten[0];
const matrix = parsed.minMax?.z || [];
const results = {};

const conrec = new Conrec(matrix, {});

try {
  try {
    console.time("Test 1");
    const result = conrec.drawContour({
      contourDrawer: "basic",
      levels: [-1000000000, 1000000000],
      timeout: 10000,
    });
    results.test1 = result.contours;
    console.timeEnd("Test 1");

    console.time("Test 2");
    const result2 = conrec.drawContour({
      contourDrawer: "basic",
      levels: [-100000, 100000],
      timeout: 10000,
    });
    results.test2 = result2.contours;
    console.timeEnd("Test 2");

    console.time("Test 3");
    const result3 = conrec.drawContour({
      contourDrawer: "basic",
      levels: [],
      timeout: 10000,
    });
    results.test3 = result3.contours;
    console.timeEnd("Test 3");

    console.time("Test 4");
    const result4 = conrec.drawContour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10000,
    });
    results.test4 = result4.contours;
    console.timeEnd("Test 4");

    console.time("Test 5");
    let matrixSwap = new Conrec(matrix, { swapAxes: true });
    const result5 = matrixSwap.drawContour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10000,
    });
    results.test5 = result5.contours;
    console.timeEnd("Test 5");

    console.time("Test 6");
    const result6 = conrec.drawContour({
      contourDrawer: "basic",
      levels: [10],
      timeout: 10,
    });
    results.test6 = result6.contours;
    console.timeEnd("Test 6");

    writeFileSync(
      `${__dirname}/contours.json`,
      JSON.stringify(results, null, 2)
    );
  } catch (err) {
    console.error("Error", err);
  }
} catch (error) {
  console.error("Main error:", error);
  console.log("\nDebug info:");
  console.log(
    "ConrecWasm methods:",
    Object.getOwnPropertyNames(Conrec.prototype)
  );
}
