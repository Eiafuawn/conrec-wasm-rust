# Execution Time Comparison

to test with no overhead: cargo run --release
to test with overhead (js is also executed): node --experimental-wasm-modules www/files.mjs

| Test | Rust Native (ms) | Rust WASM (ms) | JS (ms) | JS / Rust Native | JS / Rust WASM |
| ---- | ---------------- | -------------- | ------- | ---------------- | -------------- |
| 1    | 927.365          | 1945.0         | 1967.0  | 2.12×            | 1.01×          |
| 2    | 1068.483         | 2049.0         | 2398.0  | 2.25×            | 1.17×          |
| 3    | 0.003666         | 0.010          | 0.23    | 62.72×           | 23.00×         |
| 4    | 882.814          | 697.432        | 1612.0  | 1.83×            | 2.31×          |
| 5    | 870.391          | 684.551        | 1532.0  | 1.76×            | 2.24×          |
