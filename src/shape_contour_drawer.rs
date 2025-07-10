use crate::contour_builder::{ContourBuilder, Point};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ShapeContour {
    pub level: f64,
    k: usize,
    lines: Vec<Point>,
}

pub struct ShapeContourDrawer {
    contours: Vec<ContourBuilder>,
    swap_axes: bool,
}

impl ShapeContourDrawer {
    pub fn new(levels: Vec<f64>, swap_axes: bool) -> Self {
        let contours = levels
            .into_iter()
            .map(|level| ContourBuilder::new(level))
            .collect();

        ShapeContourDrawer {
            contours,
            swap_axes,
        }
    }

    pub fn draw_contour(&mut self, x1: f64, x2: f64, y1: f64, y2: f64, _z: f64, k: usize) {
        if !self.swap_axes {
            self.contours[k].add_segment(Point { x: y1, y: x1 }, Point { x: y2, y: x2 });
        } else {
            self.contours[k].add_segment(Point { x: x1, y: y1 }, Point { x: x2, y: y2 });
        }
    }

    pub fn get_contour(&mut self) -> Vec<ShapeContour> {
        let mut l: Vec<ShapeContour> = Vec::new();
        let a = self.contours.clone();

        for k in 0..a.len() {
            let mut s = a[k].s.clone();

            while let Some(rc_sequence) = s {
                let sequence = rc_sequence.borrow();

                let mut l2 = ShapeContour {
                    level: a[k].level,
                    k,
                    lines: Vec::new(),
                };

                let mut current = Some(sequence.head.clone());

                while let Some(node_rc) = current {
                    let node = node_rc.borrow();
                    l2.lines.push(node.p);

                    current = node.next.clone();
                }

                l.push(l2);

                s = sequence.next.clone();
            }
        }

        l
    }
}
