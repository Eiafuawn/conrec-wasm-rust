//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::{array, println};

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

use js_sys::Array;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_test::*;

extern crate conrec_wasm;
use conrec_wasm::{ utils::set_panic_hook , ConrecWasm, conrec::ConrecOptions, conrec::DrawContourOptions, conrec::ContourDrawerName };

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    set_panic_hook();
    let matrix = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
        vec![2.0, 3.0, 4.0, 5.0, 6.0],
        vec![3.0, 4.0, 5.0, 6.0, 7.0],
        vec![4.0, 5.0, 6.0, 7.0, 8.0],
        vec![5.0, 6.0, 7.0, 8.0, 9.0],
    ];

    let conrec_options = to_value(&ConrecOptions {
        swap_axes: Some(false),
        xs: Some(vec![0.0, 1.0, 2.0, 3.0, 4.0]),
        ys: Some(vec![0.0, 1.0, 2.0, 3.0, 4.0]),
    }).unwrap();
    let draw_options = to_value(&DrawContourOptions {
        levels: Some(vec![0.0, 1.0, 2.0, 3.0, 4.0]),
        nb_levels: Some(10),
        contour_drawer: Some(ContourDrawerName::Basic),
        timeout: Some(0),
    }).unwrap();
    let array_matrix = Array::new();
    for row in matrix.iter() {
        let array_row = Array::new();
        for val in row.iter() {
            array_row.push(&(*val).into());
        }
        array_matrix.push(&array_row.into());
    }
    let mut conrec = ConrecWasm::new(&array_matrix, conrec_options);
    let result = conrec.draw_contour(draw_options);
}
