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

    web_sys::console::log_1(&format!("Matrix size: {}x{}", matrix.len(), matrix[0].len()).into());

    let mut conrec = Conrec::new(matrix.clone(), None);
    web_sys::console::time_with_label("Wasm test");
    for i in 0..100 {
        let result1: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![-1000000000.0, 1000000000.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
        if i == 99 {
            log_contour_result(&result1);
        }
    }
    web_sys::console::time_end_with_label("Wasm test");

    web_sys::console::time_with_label("Wasm test 2");
    for i in 0..100 {
        let result2: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![-100000.0, 100000.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
        if i == 99 {
            log_contour_result(&result2);
        }
    }
    web_sys::console::time_end_with_label("Wasm test 2");

    web_sys::console::time_with_label("Wasm test 3");
    for i in 0..500 {
        let result3: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
        if i == 499 {
            log_contour_result(&result3);
        }
    }
    web_sys::console::time_end_with_label("Wasm test 3");

    web_sys::console::time_with_label("Wasm test 4");
    for i in 0..20 {
        let result4: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![10.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
        if i == 19 {
            log_contour_result(&result4);
        }
    }
    web_sys::console::time_end_with_label("Wasm test 4");

    web_sys::console::log_1(&JsValue::from_str("Testing with swapped axes"));
    let mut conrec_swap: Conrec = Conrec::new(
        matrix.clone(),
        Some(ConrecOptions {
            swap_axes: Some(true),
            xs: None,
            ys: None,
        }),
    );
    web_sys::console::time_with_label("Wasm test 5");
    for i in 0..20 {
        let result5: ContourResult = conrec_swap.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![10.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
        if i == 19 {
            log_contour_result(&result5);
        }
    }
    web_sys::console::time_end_with_label("Wasm test 5");

    web_sys::console::time_with_label("Wasm test 6");
    for i in 0..20 {
        let _result6: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![10.0]),
            nb_levels: Some(10),
            timeout: Some(10),
        });
        if i == 19 {
            log_contour_result(&_result6);
        }
    }
    web_sys::console::time_end_with_label("Wasm test 6");
}

fn log_contour_result(result: &ContourResult) {
    match result {
        ContourResult::Basic { contours, timeout } => {
            if contours.is_empty() {
                web_sys::console::log_1(&"No contours found".into());
                return;
            }
            web_sys::console::log_1(
                &format!("Basic contours: {:?}", contours[0].lines.len(),).into(),
            );
        }
        ContourResult::Shape { contours, timeout } => {
            if contours.is_empty() {
                web_sys::console::log_1(&"No contours found".into());
                return;
            }
            web_sys::console::log_1(
                &format!("Shape contours: {:?}", contours[0].lines.len(),).into(),
            );
        }
    }
}
