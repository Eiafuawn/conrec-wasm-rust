#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

extern crate conrec_wasm;
use conrec_wasm::process_file;

// wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn conrec() {
    process_file();
}
