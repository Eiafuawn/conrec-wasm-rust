mod basic_contour_drawer;
mod calculate_contour;
pub mod conrec;
mod contour_builder;
mod shape_contour_drawer;
pub mod utils;

use js_sys::Array;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use crate::conrec::{Conrec, ConrecOptions, ContourDrawerName, ContourResult, DrawContourOptions};
use utils::set_panic_hook;

#[wasm_bindgen]
pub struct ConrecWasm {
    conrec: Conrec,
}

#[wasm_bindgen]
impl ConrecWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(matrix: &Array, options: JsValue) -> Self {
        set_panic_hook();
        let matrix = matrix
            .iter()
            .map(|row| {
                row.dyn_into::<Array>()
                    .unwrap()
                    .iter()
                    .map(|val| val.as_f64().unwrap())
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>();

        let options = from_value(options).unwrap();
        ConrecWasm {
            conrec: Conrec::new(matrix, options),
        }
    }

    pub fn draw_contour(&mut self, options: JsValue) -> JsValue {
        let options = from_value(options).unwrap();
        let result = self.conrec.draw_contour(options);
        to_value(&result).unwrap()
    }
}

#[wasm_bindgen]
pub fn process_file() {
    console_error_panic_hook::set_once();

    let matrix: Vec<Vec<f64>> = serde_json::from_str(std::include_str!("../www/matrix.json"))
        .expect("Failed to parse matrix.json");

    let mut conrec = Conrec::new(matrix.clone(), None);
    for _ in 0..100 {
        let _result1: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![-1000000000.0, 1000000000.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
    }

    for _ in 0..100 {
        let _result2: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![-100000.0, 100000.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
    }

    for _ in 0..500 {
        let _result3: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
    }

    for _ in 0..20 {
        let _result4: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![10.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
    }

    let mut conrec_swap: Conrec = Conrec::new(
        matrix.clone(),
        Some(ConrecOptions {
            swap_axes: Some(true),
            xs: None,
            ys: None,
        }),
    );
    for _ in 0..20 {
        let _result5: ContourResult = conrec_swap.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![10.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
    }

    for _ in 0..20 {
        let _result6: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![10.0]),
            nb_levels: Some(10),
            timeout: Some(10),
        });
    }
}
