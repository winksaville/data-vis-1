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
    use linregress::assert_almost_eq;

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

    #[test]
    fn test_36_xz_values() {
        // Get the mode print various values
        let model = my_model();
        let parameters: Vec<_> = model.iter_parameter_pairs().collect();
        println!("parameters: {parameters:?}");
        let pvalues: Vec<_> = model.iter_p_value_pairs().collect();
        println!("pvalues: {pvalues:?}");
        let standard_errors: Vec<_> = model.iter_se_pairs().collect();
        println!("standard_errors: {standard_errors:?}");

        // Precompute the predictions for the test data
        let mut test_data: Vec<(&str, Vec<f64>)> = vec![];
        let mut inner = 0;
        for a in (-3..3).map(|f| f as f64 / 10.0) {
            for b in (-3..3).map(|f| f as f64 / 10.0) {
                inner = inner + 1;
                let mut data = vec![("x", vec![a]), ("z", vec![b])];
                test_data.append(&mut data);
            }
        }
        println!("inner: {inner} test_data.len(): {} x: {:?}", test_data.len(), test_data);
        assert_eq!(test_data.len(), inner * 2);

        // Now predict the values, I expected {inner} number of predictions but there was only 1?
        let predictions = model.predict(test_data).unwrap();
        println!("predictions.len(): {} predictions: {:0.2?} WHY is predections.len() only 1, expecting {inner}?", predictions.len(), predictions);
        assert_eq!(predictions.len(), 1); // This should fail
        assert_ne!(predictions.len(), inner); // I was expecting to pass

        //
        for a in (-3..3).map(|f| f as f64 / 10.0) {
            for b in (-3..3).map(|f| f as f64 / 10.0) {
                let data = vec![("x", vec![a]), ("z", vec![b])];
                let x = model.predict(data.clone()).unwrap();
                assert_almost_eq!(x[0], 1.0, 1e-2);
                //println!("x.len(): {} x: {:0.2?}", x.len(), x);
            }
        }
    }
}
