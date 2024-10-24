#[derive(Debug, Clone)]
struct BasicContour {
    z_value: f64,
    lines: Vec<f64>,
}

#[derive(Clone)]
struct BasicContourDrawer {
    contour: Vec<BasicContour>,
    swap_axes: bool,
}

impl BasicContourDrawer {
    fn new(levels: Vec<f64>, swap_axes: bool) -> Self {
        let mut contour = Vec::new();
        for level in levels {
            contour.push(BasicContour {
                z_value: level,
                lines: Vec::new(),
            });
        }
        BasicContourDrawer {
            contour: contour,
            swap_axes,
        }
    }
    
    fn draw_contour(
        &mut self,
        x1: f64,
        x2: f64,
        y1: f64,
        y2: f64,
        z: f64,
        k: usize,
    ) {
        if !self.swap_axes {
            self.contour[k].lines.extend(vec![y1, x1, y2, x2]);
        } else {
            self.contour[k].lines.extend(vec![x1, y1, x2, y2]);
        }
    }
    
    fn get_contour(&self) -> Vec<BasicContour> {
        self.contour.clone()
    }
}