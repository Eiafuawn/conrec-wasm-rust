mod utils;
mod contour_builder;
mod basic_contour_drawer;
mod shape_contour_drawer;
mod calculate_contour;
mod number_types;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, conrec-wasm!");
}
