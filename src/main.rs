mod basic_contour_drawer;
mod calculate_contour;
mod conrec;
mod contour_builder;
mod shape_contour_drawer;
mod utils;

use conrec::{Conrec, ConrecOptions, ContourDrawerName, ContourResult, DrawContourOptions};
use std::time::Instant;
use utils::set_panic_hook;

pub fn main() {
    set_panic_hook();
    let matrix: Vec<Vec<f64>> =
        serde_json::from_str(std::include_str!("../www/matrix.json")).unwrap();

    let mut conrec: Conrec = Conrec::new(matrix.clone(), None);
    let timer = Instant::now();
    for _ in 0..100 {
        let _result1: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![-1000000000.0, 1000000000.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
    }
    let elapsed = timer.elapsed();
    println!("Test 1: {:?}", elapsed);

    let timer2 = Instant::now();
    for _ in 0..100 {
        let _result2: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![-100000.0, 100000.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
    }
    println!("Test 2: {:?}", timer2.elapsed());

    let timer3 = Instant::now();
    for _ in 0..500 {
        let _result3: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
    }
    println!("Test 3: {:?}", timer3.elapsed());

    let timer4 = Instant::now();
    for _ in 0..20 {
        let _result4: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![10.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
    }
    println!("Test 4: {:?}", timer4.elapsed());

    let mut conrec_swap: Conrec = Conrec::new(
        matrix,
        Some(ConrecOptions {
            swap_axes: Some(true),
            xs: None,
            ys: None,
        }),
    );
    let timer5 = Instant::now();
    for _ in 0..20 {
        let _result5: ContourResult = conrec_swap.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![10.0]),
            nb_levels: Some(10),
            timeout: Some(10000),
        });
    }
    println!("Test 5: {:?}", timer5.elapsed());

    let timer6 = Instant::now();
    for _ in 0..20 {
        let _result6: ContourResult = conrec.draw_contour(DrawContourOptions {
            contour_drawer: Some(ContourDrawerName::Basic),
            levels: Some(vec![10.0]),
            nb_levels: Some(10),
            timeout: Some(10),
        });
    }
    println!("Test 6: {:?}", timer6.elapsed());
}
