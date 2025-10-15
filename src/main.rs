mod linear_regression;
use linear_regression::SimpleLinearRegression;

fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0]; // Independent variable
    let y = vec![2.0, 4.0, 5.0, 4.0, 5.0]; // Dependent variable

    let (slope, intercept) = SimpleLinearRegression::compute(&x, &y);

    let predictions = SimpleLinearRegression::predict(x.clone(), slope, intercept);
    println!("Predictions: {:?}", predictions);

    let mse = SimpleLinearRegression::mean_squared_error(y, predictions);
    println!("Mean Squared Error: {:?}", mse);
}
