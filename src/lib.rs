mod utils;
mod contour_builder;
mod basic_contour_drawer;
mod shape_contour_drawer;
mod calculate_contour;
pub mod conrec;

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};
use js_sys::{Array, Object, Reflect};
use serde::{Serialize, Deserialize};

use crate::basic_contour_drawer::{BasicContourDrawer, BasicContour};
use crate::shape_contour_drawer::{ShapeContourDrawer, ShapeContour};
use crate::conrec::{Conrec, ConrecOptions, ContourDrawer, ContourDrawerName, ContourResult, DrawContourOptions};


#[wasm_bindgen]
pub struct ConrecWasm {
    conrec: Conrec,
}

#[wasm_bindgen]
impl ConrecWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(matrix: &Array, options: JsValue) -> Self {
      let matrix = matrix.iter().map(|row| {
          row.dyn_into::<Array>().unwrap().iter().map(|val| {
              val.as_f64().unwrap()
          }).collect::<Vec<f64>>()
      }).collect::<Vec<Vec<f64>>>();
    
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