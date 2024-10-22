mod utils;
mod contour_builder;
mod basic_contour_drawer;
mod shape_contour_drawer;
mod calculate_contour;

use wasm_bindgen::prelude::*;

use contour_builder::ContourBuilder;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, conrec-wasm!");
}
