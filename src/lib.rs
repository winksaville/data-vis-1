use linregress::{FormulaRegressionBuilder, RegressionDataBuilder, RegressionModel};

pub fn my_model() -> RegressionModel {
    // Given data points as (x, y, z) tuples
    let points = [
        (1.0, 1.0, 0.0),
        (-1.0, 1.0, 0.0),
        (0.0, 1.0, 1.0),
        (0.5, 1.0, 0.5),
    ];

    let x: Vec<_> = points.iter().map(|p| p.0).collect();
    let y: Vec<_> = points.iter().map(|p| p.1).collect();
    let z: Vec<_> = points.iter().map(|p| p.2).collect();

    // Constructing data using a vector of tuples
    let data_tuples = vec![("x", x), ("y", y), ("z", z)];

    // Setup regression data
    let data = RegressionDataBuilder::new()
        .build_from(data_tuples)
        .unwrap();

    // Define the regression formula
    let formula = "y ~ x + z";
    let model = FormulaRegressionBuilder::new()
        .data(&data)
        .formula(formula)
        .fit()
        .unwrap();

    model
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let model = my_model();

        // Predict y for the test kkkoint (0, 0.5)
        let test_data = vec![("x", vec![0.0]), ("z", vec![0.5])];
        let predictions: Vec<f64> = model.predict(test_data.clone()).unwrap();

        let x = test_data.get(0).unwrap().1[0];
        let z = test_data.get(1).unwrap().1[0];

        let y_predicted = predictions.get(0).expect("No prediction found");
        println!("Predicted y value for ({}, {}): {}", x, z, y_predicted);
        assert_eq!(*y_predicted, 1.0);
    }
}
