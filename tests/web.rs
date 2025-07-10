#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use core::time;
use std::{array, println, vec};

extern crate web_sys;
use wasm_bindgen::JsValue;
use web_sys::console;

use js_sys::Array;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_test::*;

extern crate conrec_wasm;
use conrec_wasm::{ conrec::{Conrec, ConrecOptions, ContourDrawer, ContourDrawerName, ContourResult, DrawContourOptions}, utils::set_panic_hook, ConrecWasm};

// wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn conrec() {
    set_panic_hook();
    
    /* 
    let matrix: Vec<Vec<f64>> = serde_json::from_str(std::include_str!("../www/matrix.json")).unwrap();
    let mut conrec: Conrec = Conrec::new(matrix.clone(), None);
    
    let mut timer = Timer::new("test 1");
    for i in 0..50 {
        let result1: ContourResult = conrec.draw_contour(DrawContourOptions { 
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![-1000000000.0, 1000000000.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
    }
    timer.stop();
    
    let mut timer = Timer::new("test 2");
    for i in 0..50 {
    let result2: ContourResult = conrec.draw_contour(DrawContourOptions { 
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![-100000.0, 100000.0]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });
}
    timer.stop();

    let mut timer = Timer::new("test 3");
    for i in 0..100 {
    let result3: ContourResult = conrec.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });
}
    timer.stop();   

    let mut timer = Timer::new("test 4");
    for i in 0..10 {
    let result4: ContourResult = conrec.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![10.0]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });
}
    timer.stop();

    let mut conrec_swap: Conrec = Conrec::new(matrix, 
        Some(ConrecOptions { 
            swap_axes: Some(true),
            xs: None,
            ys: None
        })
    );
        let mut timer = Timer::new("test 5");
    for i in 0..10 {
    let result5: ContourResult = conrec_swap.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![10.0]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });
}
    timer.stop();

    let mut timer = Timer::new("test 6");
    for i in 0..10 {
    let result6: ContourResult = conrec.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![10.0]),   
        nb_levels: Some(10),
        timeout: Some(10),
    });
}
    timer.stop();
} */

/* #[wasm_bindgen_test]
fn conrecWasm() {
    set_panic_hook();
    let matrix: Vec<Vec<f64>> = serde_json::from_str(std::include_str!("../www/matrix.json")).unwrap();
    let array: Array = Array::from(to_value(&matrix).unwrap()).unwrap();
    
    let mut conrec: ConrecWasm = ConrecWasm::new(&array.clone(), None);
    let result1: ContourResult = conrec.draw_contour(&JsValue::from_serde(DrawContourOptions { 
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![-1000000000.0, 1000000000.0]),
        nb_levels: Some(10),
        timeout: Some(10000),
    }));
    
    let mut timer = Timer::new("draw_contour");
    let result2: ContourResult = conrec.draw_contour(DrawContourOptions { 
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![-100000.0, 100000.0]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });
    timer.stop();

    let result3: ContourResult = conrec.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });

    let result4: ContourResult = conrec.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![10.0]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });

    let mut conrec_swap: Conrec = Conrec::new(&array, 
        Some(JsValue::from_serde(&ConrecOptions { 
            swap_axes: Some(true),
            xs: None,
            ys: None
        }))
    );
    let result5: ContourResult = conrec_swap.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![10.0]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });

    let result6: ContourResult = conrec.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![10.0]),   
        nb_levels: Some(10),
        timeout: Some(10),
    });
   
} */
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct Timer<'a> {
    name: &'a str,
    is_stopped: bool,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name, is_stopped: false }
    }

    pub fn stop(&mut self) {
        if !self.is_stopped {
            console::time_end_with_label(self.name);
            self.is_stopped = true;
        }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        if !self.is_stopped {
            // Automatically stop the timer if it hasn't been stopped yet
            console::time_end_with_label(self.name);
        }
    }
}

