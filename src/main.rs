
mod conrec;
mod utils;
mod basic_contour_drawer;
mod shape_contour_drawer;
mod calculate_contour;
mod contour_builder;

use conrec::{Conrec, ConrecOptions, DrawContourOptions, ContourResult, ContourDrawerName};
use utils::set_panic_hook;
use core::time;
use std::time::Instant;

pub fn main() {
    set_panic_hook();
    let matrix: Vec<Vec<f64>> = serde_json::from_str(std::include_str!("../www/matrix.json")).unwrap();
    
    let mut conrec: Conrec = Conrec::new(matrix.clone(), None);
    let timer = Instant::now();
    let result1: ContourResult = conrec.draw_contour(DrawContourOptions { 
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![-1000000000.0, 1000000000.0]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });
    let elapsed = timer.elapsed();
    println!("Time elapsed in draw_contour() is: {:?}", elapsed);
    
    let timer2 = Instant::now();
    let result2: ContourResult = conrec.draw_contour(DrawContourOptions { 
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![-100000.0, 100000.0]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });
    println!("Time elapsed in draw_contour() is: {:?}", timer2.elapsed());

    let timer3 = Instant::now();
    let result3: ContourResult = conrec.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });
    println!("Time elapsed in draw_contour() is: {:?}", timer3.elapsed());

    let timer4 = Instant::now();
    let result4: ContourResult = conrec.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![10.0]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });
    println!("Time elapsed in draw_contour() is: {:?}", timer4.elapsed());

    let mut conrec_swap: Conrec = Conrec::new(matrix, 
        Some(ConrecOptions { 
            swap_axes: Some(true),
            xs: None,
            ys: None
        })
    );
    let timer5 = Instant::now();
    let result5: ContourResult = conrec_swap.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![10.0]),
        nb_levels: Some(10),
        timeout: Some(10000),
    });
    println!("Time elapsed in draw_contour() is: {:?}", timer5.elapsed());

    let timer6 = Instant::now();
    let result6: ContourResult = conrec.draw_contour(DrawContourOptions {
        contour_drawer: Some(ContourDrawerName::Basic),
        levels: Some(vec![10.0]),   
        nb_levels: Some(10),
        timeout: Some(10),
    });
    print!("Time elapsed in draw_contour() is: {:?}", timer6.elapsed());
}