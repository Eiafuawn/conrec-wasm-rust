extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const EPSILON: f64 = 1e-10;
const MINUS_EPSILON: f64 = -1e-10;

pub trait ContourDrawer {
    fn draw_contour(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, z: f64, k: usize);
}

#[derive(Default)]
pub struct CalculateContourOptions {
    pub timeout: Option<u64>,
    pub ilb: Option<usize>,
    pub iub: Option<usize>,
    pub jlb: Option<usize>,
    pub jub: Option<usize>,
}

// Helper functions moved outside to avoid borrowing issues
fn xsect(h: &[f64], xh: &[f64], p1: usize, p2: usize) -> f64 {
    (h[p2] * xh[p1] - h[p1] * xh[p2]) / (h[p2] - h[p1])
}

fn ysect(h: &[f64], yh: &[f64], p1: usize, p2: usize) -> f64 {
    (h[p2] * yh[p1] - h[p1] * yh[p2]) / (h[p2] - h[p1])
}

pub fn calculate_contour(
    matrix: &Vec<Vec<f64>>,
    x: &[f64],
    y: &[f64],
    z: &[f64],
    contour_drawer: &mut dyn ContourDrawer,
    options: Option<CalculateContourOptions>,
) -> Result<bool, String> {
    // Input validation

    let options = options.unwrap_or_default();
    let ilb = options.ilb.unwrap_or(0);
    let iub = options.iub.unwrap_or(matrix.len() - 1);
    let jlb = options.jlb.unwrap_or(0);
    let jub = options.jub.unwrap_or(matrix[0].len() - 1);

    // Pre-allocate vectors to avoid reallocation
    let mut h = vec![0.0; 5];
    let mut sh = vec![0i8; 5];
    let mut xh = vec![0.0; 5];
    let mut yh = vec![0.0; 5];

    let nc = z.len();
    let z0 = z[0];
    let znc1 = z[nc - 1];

    // The indexing arrays
    let im: [usize; 4] = [0, 1, 1, 0];
    let jm: [usize; 4] = [0, 0, 1, 1];

    // 3D lookup table for case values
    let castab: [[[i32; 3]; 3]; 3] = [
        [[0, 0, 8], [0, 2, 5], [7, 6, 9]],
        [[0, 3, 4], [1, 3, 1], [4, 3, 0]],
        [[9, 6, 7], [5, 2, 0], [8, 0, 0]],
    ];

    for j in (jlb..jub).rev() {
        for i in ilb..iub {
            // Bounds checking
            if i + 1 >= matrix.len() || j + 1 >= matrix[i].len() {
                continue;
            }

            let dij = matrix[i][j];
            let dij1 = matrix[i][j + 1];
            let di1j = matrix[i + 1][j];
            let di1j1 = matrix[i + 1][j + 1];

            let (min1, max1) = if dij > dij1 {
                (dij1, dij)
            } else {
                (dij, dij1)
            };

            let (min2, max2) = if di1j > di1j1 {
                (di1j1, di1j)
            } else {
                (di1j, di1j1)
            };

            let dmin = min1.min(min2);
            let dmax = max1.max(max2);

            if dmax >= z0 && dmin <= znc1 {
                for k in 0..nc {
                    if z[k] >= dmin && z[k] <= dmax {
                        // Fill corner values
                        h[1] = matrix[i][j] - z[k];
                        h[2] = matrix[i + 1][j] - z[k];
                        h[3] = matrix[i + 1][j + 1] - z[k];
                        h[4] = matrix[i][j + 1] - z[k];

                        // Fill corner coordinates
                        xh[1] = x[i];
                        xh[2] = x[i + 1];
                        xh[3] = x[i + 1];
                        xh[4] = x[i];

                        yh[1] = y[j];
                        yh[2] = y[j];
                        yh[3] = y[j + 1];
                        yh[4] = y[j + 1];

                        // Calculate center point
                        h[0] = 0.25 * (h[1] + h[2] + h[3] + h[4]);
                        xh[0] = 0.5 * (x[i] + x[i + 1]);
                        yh[0] = 0.5 * (y[j] + y[j + 1]);

                        // Calculate signs
                        for m in 0..=4 {
                            sh[m] = if h[m] > EPSILON {
                                1
                            } else if h[m] < MINUS_EPSILON {
                                -1
                            } else {
                                0
                            };
                        }

                        // Process each triangle
                        for m in 1..=4 {
                            let m1 = m;
                            let m2 = 0;
                            let m3 = if m != 4 { m + 1 } else { 1 };

                            let case_value = castab[(sh[m1] + 1) as usize][(sh[m2] + 1) as usize][(sh[m3] + 1) as usize];

                            if case_value != 0 {
                                let (x1, y1, x2, y2) = match case_value {
                                    1 => (xh[m1], yh[m1], xh[m2], yh[m2]),
                                    2 => (xh[m2], yh[m2], xh[m3], yh[m3]),
                                    3 => (xh[m3], yh[m3], xh[m1], yh[m1]),
                                    4 => (xh[m1], yh[m1], xsect(&h, &xh, m2, m3), ysect(&h, &yh, m2, m3)),
                                    5 => (xh[m2], yh[m2], xsect(&h, &xh, m3, m1), ysect(&h, &yh, m3, m1)),
                                    6 => (xh[m3], yh[m3], xsect(&h, &xh, m1, m2), ysect(&h, &yh, m1, m2)),
                                    7 => (xsect(&h, &xh, m1, m2), ysect(&h, &yh, m1, m2), xsect(&h, &xh, m2, m3), ysect(&h, &yh, m2, m3)),
                                    8 => (xsect(&h, &xh, m2, m3), ysect(&h, &yh, m2, m3), xsect(&h, &xh, m3, m1), ysect(&h, &yh, m3, m1)),
                                    9 => (xsect(&h, &xh, m3, m1), ysect(&h, &yh, m3, m1), xsect(&h, &xh, m1, m2), ysect(&h, &yh, m1, m2)),
                                    _ => continue,
                                };

                                contour_drawer.draw_contour(x1, y1, x2, y2, z[k], k);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(false)
}