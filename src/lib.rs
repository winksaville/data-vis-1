use nalgebra as na;
use lazy_static::lazy_static;

pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

// private use scanc02_points()
lazy_static! {
    // Sanden SANCO2 from page 13 of this manual:
    //   https://static1.squarespace.com/static/5c1a79ca96d455dcbffdc742/t/5c474cee562fa759dd733b04/1548176625850/Sanden_sanc02_technical-info_10-2017_4.pdf
    static ref SANCO2_POINTS: [Point3D; 29] = [
        Point3D::new(-13.0, 1.75, 140.0),
        Point3D::new(-13.0, 1.7, 150.0),
        Point3D::new(-13.0, 1.6, 160.0),
        Point3D::new(-4.0, 2.0, 140.0),
        Point3D::new(-4.0, 1.8, 150.0),
        Point3D::new(-4.0, 1.9, 160.0),
        Point3D::new(5.0, 2.2, 140.0),
        Point3D::new(5.0, 2.2, 150.0),
        Point3D::new(5.0, 2.1, 160.0),
        Point3D::new(5.0, 1.95, 175.0),
        Point3D::new(35.0, 3.35, 140.0),
        Point3D::new(35.0, 3.55, 150.0),
        Point3D::new(35.0, 3.45, 160.0),
        Point3D::new(35.0, 3.2, 175.0),
        Point3D::new(44.0, 4.25, 140.0),
        Point3D::new(44.0, 4.1, 150.0),
        Point3D::new(44.0, 3.9, 160.0),
        Point3D::new(44.0, 3.55, 175.0),
        Point3D::new(68.0, 5.2, 140.0),
        Point3D::new(68.0, 4.75, 150.0),
        Point3D::new(68.0, 4.4, 160.0),
        Point3D::new(68.0, 3.9, 175.0),
        Point3D::new(77.0, 4.9, 140.0),
        Point3D::new(77.0, 4.6, 150.0),
        Point3D::new(77.0, 4.35, 160.0),
        Point3D::new(77.0, 4.0, 175.0),
        Point3D::new(108.0, 3.8, 140.0),
        Point3D::new(108.0, 4.0, 150.0),
        Point3D::new(108.0, 3.95, 160.0),
    ];
}

pub fn sanco2_points() -> &'static [Point3D] {
    &*SANCO2_POINTS
}

pub fn polynomial_regression(points: &[Point3D]) -> na::DVector<f64> {
    let n = points.len();
    let mut design_matrix = na::DMatrix::<f64>::zeros(n, 10);
    let mut dependent_var = na::DVector::<f64>::zeros(n);

    for (i, point) in points.iter().enumerate() {
        design_matrix[(i, 0)] = point.x.powi(3);
        design_matrix[(i, 1)] = point.x.powi(2) * point.z;
        design_matrix[(i, 2)] = point.x * point.z.powi(2);
        design_matrix[(i, 3)] = point.z.powi(3);
        design_matrix[(i, 4)] = point.x.powi(2);
        design_matrix[(i, 5)] = point.x * point.z;
        design_matrix[(i, 6)] = point.z.powi(2);
        design_matrix[(i, 7)] = point.x;
        design_matrix[(i, 8)] = point.z;
        design_matrix[(i, 9)] = 1.0;
        dependent_var[i] = point.y;
    }

    let xt = design_matrix.transpose();
    #[allow(clippy::let_and_return)]
    let coefficients = (xt.clone() * design_matrix).try_inverse().unwrap() * xt * dependent_var;
    //println!("polynomial_regression:- coefficients: {coefficients:?}");
    coefficients
}

// Predict Y value
pub fn predict_y(x: f64, z: f64, coeffs: &na::DVector<f64>) -> f64 {
    coeffs[0] * x.powi(3)
        + coeffs[1] * x.powi(2) * z
        + coeffs[2] * x * z.powi(2)
        + coeffs[3] * z.powi(3)
        + coeffs[4] * x.powi(2)
        + coeffs[5] * x * z
        + coeffs[6] * z.powi(2)
        + coeffs[7] * x
        + coeffs[8] * z
        + coeffs[9]
}

pub fn mean_squared_error(points: &[Point3D], coeffs: &na::DVector<f64>) -> f64 {
    let number_points = points.len() as f64;
    points
        .iter()
        .map(|point| (point.y - predict_y(point.x, point.y, coeffs)).powi(2))
        .sum::<f64>()
        / number_points
}

pub fn sanco2_coeffs() -> na::DVector<f64> {
    polynomial_regression(sanco2_points())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let coeffs = sanco2_coeffs();

        // A simple test might be to ensure the MSE is below a certain threshold
        let mse = mean_squared_error(sanco2_points(), &coeffs);
        println!("test_regression: mse={mse}");
        assert!(mse < 1.0); // Not good enough if it fails
    }
}
