# Execution Time Comparison

to test with no overhead: cargo run --release
to test with overhead (js is also executed): node --experimental-wasm-modules www/files.mjs

| Test | Rust (no overhead) | Rust (with overhead) | JS (WASM) | Rust (no overhead) vs JS | Rust (with overhead) vs JS |
| ---- | ------------------ | -------------------- | --------- | ------------------------ | -------------------------- |
| 1    | 1.868s             | 4.541s               | 4.183s    | -55.3%                   | +8.6%                      |
| 2    | 2.187s             | 5.076s               | 4.342s    | -49.6%                   | +16.9%                     |
| 3    | 0.000019s          | 0.000033s            | 0.000215s | -91.2%                   | -84.7%                     |
| 4    | 2.109s             | 2.312s               | 1.983s    | +6.4%                    | +16.6%                     |
| 5    | 2.131s             | 2.302s               | 2.015s    | +5.8%                    | +14.2%                     |
| 6    | 2.084s             | 2.306s               | 0.220s    | +847.3%                  | +948.2%                    |
