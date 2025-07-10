use serde::{Deserialize, Serialize};

use crate::basic_contour_drawer::{BasicContour, BasicContourDrawer};
use crate::calculate_contour::{
    calculate_contour, CalculateContourOptions, ContourDrawer as CalcContourDrawer,
};
use crate::shape_contour_drawer::{ShapeContour, ShapeContourDrawer};

extern crate web_sys;

#[derive(Serialize, Deserialize)]
pub struct ConrecOptions {
    pub xs: Option<Vec<f64>>,
    pub ys: Option<Vec<f64>>,
    pub swap_axes: Option<bool>,
}

impl Default for ConrecOptions {
    fn default() -> Self {
        ConrecOptions {
            xs: None,
            ys: None,
            swap_axes: None,
        }
    }
}

pub enum ContourDrawer {
    Basic(BasicContourDrawer),
    Shape(ShapeContourDrawer),
}

impl CalcContourDrawer for ContourDrawer {
    fn draw_contour(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, z: f64, k: usize) {
        match self {
            ContourDrawer::Basic(drawer) => drawer.draw_contour(x1, y1, x2, y2, z, k),
            ContourDrawer::Shape(drawer) => drawer.draw_contour(x1, y1, x2, y2, z, k),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ContourDrawerName {
    Basic,
    Shape,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ContourResult {
    Basic {
        contours: Vec<BasicContour>,
        timeout: bool,
    },
    Shape {
        contours: Vec<ShapeContour>,
        timeout: bool,
    },
}

#[derive(Serialize, Deserialize)]
pub struct DrawContourOptions {
    pub levels: Option<Vec<f64>>,
    pub nb_levels: Option<usize>,
    pub contour_drawer: Option<ContourDrawerName>,
    pub timeout: Option<u64>,
}

pub struct Conrec {
    pub matrix: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    pub swap_axes: bool,
    pub has_min_max: bool,
    pub min: f64,
    pub max: f64,
}

impl Conrec {
    pub fn new(matrix: Vec<Vec<f64>>, options: Option<ConrecOptions>) -> Self {
        let swap_axes = options
            .as_ref()
            .map_or(false, |o| o.swap_axes.unwrap_or(false));
        let rows = matrix.len();
        let cols = matrix.first().map_or(0, |row| row.len());

        // Helper function to generate range similar to JS range(start, end, step)
        fn range(start: usize, end: usize, step: usize) -> Vec<f64> {
            (start..end).step_by(step).map(|i| i as f64).collect()
        }

        // Handle xs and ys based on swap_axes, matching JS behavior
        let (xs, ys) = if swap_axes {
            // When axes are swapped, xs are in rows direction
            (
                options
                    .as_ref()
                    .and_then(|o| o.xs.clone())
                    .unwrap_or_else(|| range(0, rows, 1)),
                options
                    .as_ref()
                    .and_then(|o| o.ys.clone())
                    .unwrap_or_else(|| range(0, cols, 1)),
            )
        } else {
            // When axes are not swapped, we swap the internal values
            (
                options
                    .as_ref()
                    .and_then(|o| o.ys.clone())
                    .unwrap_or_else(|| range(0, rows, 1)),
                options
                    .as_ref()
                    .and_then(|o| o.xs.clone())
                    .unwrap_or_else(|| range(0, cols, 1)),
            )
        };

        Conrec {
            matrix,
            rows,
            cols,
            xs,
            ys,
            swap_axes,
            has_min_max: false,
            min: 0.0,
            max: 0.0,
        }
    }

    pub fn draw_contour(&mut self, options: DrawContourOptions) -> ContourResult {
        let nb_levels = options.nb_levels.unwrap_or(10);
        let timeout = options.timeout.unwrap_or(0);
        let contour_drawer = options.contour_drawer.unwrap_or(ContourDrawerName::Basic);
        let levels = if let Some(l) = options.levels {
            l
        } else {
            self._compute_min_max();
            let interval = (self.max - self.min) / (nb_levels as f64 - 1.0);
            let mut l = range(self.min, self.max, interval);
            l.sort_by(|a, b| a.partial_cmp(b).unwrap());
            l
        };

        let mut drawer = match contour_drawer {
            ContourDrawerName::Basic => {
                ContourDrawer::Basic(BasicContourDrawer::new(levels.clone(), self.swap_axes))
            }
            ContourDrawerName::Shape => {
                ContourDrawer::Shape(ShapeContourDrawer::new(levels.clone(), self.swap_axes))
            }
        };

        let calculate_options = CalculateContourOptions {
            timeout: Some(timeout),
            jlb: None,
            jub: None,
            ilb: None,
            iub: None,
        };

        let is_timeout = calculate_contour(
            &self.matrix,
            &self.xs,
            &self.ys,
            &levels,
            &mut drawer,
            Some(calculate_options),
        );

        match &mut drawer {
            ContourDrawer::Basic(basic_drawer) => ContourResult::Basic {
                contours: basic_drawer.get_contour(),
                timeout: is_timeout.unwrap(),
            },
            ContourDrawer::Shape(shape_drawer) => ContourResult::Shape {
                contours: shape_drawer.get_contour(),
                timeout: is_timeout.unwrap(),
            },
        }
    }

    fn _compute_min_max(&mut self) {
        if !self.has_min_max {
            let (min, max) = min_max(&self.matrix);
            self.min = min;
            self.max = max;
            self.has_min_max = true;
        }
    }
}

fn range(from: f64, to: f64, step: f64) -> Vec<f64> {
    let mut result = Vec::new();
    let mut i = from;
    while i <= to {
        result.push(i);
        i += step;
    }
    result
}

fn min_max(matrix: &Vec<Vec<f64>>) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for row in matrix {
        for &val in row {
            min = min.min(val);
            max = max.max(val);
        }
    }
    (min, max)
}
