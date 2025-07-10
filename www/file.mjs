import * as wasm from "conrec-wasm";
import { Conrec } from "ml-conrec";
import { readFileSync } from "fs";
import { dirname } from "path";
import { fileURLToPath } from "url";
import { convert } from "jcampconverter";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const data = readFileSync(`${__dirname}/zhmbc_0.jdx`, "utf8");
const parsed = convert(data, { noContour: true }).flatten[0];
const matrix = parsed.minMax?.z || [];

wasm.process_file();

const conrec = new Conrec(matrix, {});
console.time("Test 1");
for (let i = 0; i < 100; i++) {
  const result = conrec.drawContour({
    contourDrawer: "basic",
    levels: [-1000000000, 1000000000],
    timeout: 10000,
  });
  if (i === 99) {
    console.log("Result:", result.contours[0].lines.length);
  }
}
console.timeEnd("Test 1");

console.time("Test 2");
for (let i = 0; i < 100; i++) {
  const result2 = conrec.drawContour({
    contourDrawer: "basic",
    levels: [-100000, 100000],
    timeout: 10000,
  });
  if (i === 99) {
    console.log("Result 2:", result2.contours[0].lines.length);
  }
}
console.timeEnd("Test 2");

console.time("Test 3");
for (let i = 0; i < 500; i++) {
  const result3 = conrec.drawContour({
    contourDrawer: "basic",
    levels: [],
    timeout: 10000,
  });
  if (i === 499) {
    console.log("Result 3: no contours:", result3.contours.length);
  }
}
console.timeEnd("Test 3");

console.time("Test 4");
for (let i = 0; i < 20; i++) {
  const result4 = conrec.drawContour({
    contourDrawer: "basic",
    levels: [10],
    timeout: 10000,
  });
  if (i === 19) {
    console.log("Result 4:", result4.contours[0].lines.length);
  }
}
console.timeEnd("Test 4");

let matrixSwap = new Conrec(matrix, { swapAxes: true });
console.time("Test 5");
for (let i = 0; i < 20; i++) {
  const result5 = matrixSwap.drawContour({
    contourDrawer: "basic",
    levels: [10],
    timeout: 10000,
  });
  if (i === 19) {
    console.log("Result 5:", result5.contours[0].lines.length);
  }
}
console.timeEnd("Test 5");

console.time("Test 6");
for (let i = 0; i < 20; i++) {
  const result6 = conrec.drawContour({
    contourDrawer: "basic",
    levels: [10],
    timeout: 10,
  });
  if (i === 19) {
    console.log("Result 6:", result6.contours[0].lines.length);
  }
}
console.timeEnd("Test 6");
