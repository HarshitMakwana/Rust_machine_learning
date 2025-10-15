pub struct SimpleLinearRegression;

impl SimpleLinearRegression {
    // Its only take one Column as x and one Column as y
    pub fn mean(x: &Vec<f64>) -> f64 {
        let sum: f64 = x.iter().sum();
        sum / x.len() as f64
    }

  /// The `covariance` function calculates the covariance between two vectors of f64 values given their
  /// means.
  /// 
  /// Arguments:
  /// 
  /// * `x`: A reference to a vector of f64 values representing one variable.
  /// * `y`: The `y` parameter in the `covariance` function represents a vector of f64 values. It is
  /// used to calculate the covariance between two sets of data represented by vectors `x` and `y`.
  /// * `x_mean`: The `x_mean` parameter represents the mean value of the elements in vector `x`.
  /// * `y_mean`: The `y_mean` parameter represents the mean value of the `y` vector. It is used in the
  /// covariance calculation to determine the deviation of each element in the `y` vector from its mean.
  /// 
  /// Returns:
  /// 
  /// The function `covariance` calculates the covariance between two vectors `x` and `y` based on their
  /// means `x_mean` and `y_mean`. It returns the covariance value as a `f64` type.
    pub fn covariance(x: &Vec<f64>, y: &Vec<f64>, x_mean: &f64, y_mean: &f64) -> f64 {
        x.iter()
            .zip(y.iter())
            .map(|(&xi, &yi)| (xi - x_mean) * (yi - y_mean))
            .sum::<f64>()
            / x.len() as f64
    }

    pub fn variance(x: &Vec<f64>, x_mean: &f64) -> f64 {
        x.iter()
            .map(|xi| (xi - x_mean).powi(2))
            .sum::<f64>()
            / x.len() as f64
    }

/// The `compute` function calculates the slope and intercept of a linear regression model given two
/// vectors of data points.
/// 
/// Arguments:
/// 
/// * `x`: The `x` parameter is a reference to a vector of `f64` values.
/// * `y`: The `y` parameter in the `compute` function is a reference to a vector of `f64` values. It
/// represents the dependent variable values in a linear regression model.
    pub fn compute(x: &Vec<f64>, y: &Vec<f64>) -> (f64, f64) {
        let x_mean = Self::mean(&x);
        let y_mean = Self::mean(&y);

        let m = Self::covariance(&x, &y, &x_mean, &y_mean) / Self::variance(x, &x_mean);
        let b = y_mean - (m * x_mean);
        (m, b)
    }

 /// The function `predict` takes a vector of f64 values, a slope, and an intercept, and returns a
 /// vector of predicted values using the linear equation y = slope*x + intercept.
 /// 
 /// Arguments:
 /// 
 /// * `x`: A vector of f64 values representing the input data points for which you want to make
 /// predictions.
 /// * `slope`: The slope parameter represents the slope of the linear equation used for prediction. It
 /// determines the rate at which the predicted values change with respect to the input values.
 /// * `intercept`: The intercept is the value where the regression line crosses the y-axis. It
 /// represents the predicted value of the dependent variable when the independent variable is zero.
 /// 
 /// Returns:
 /// 
 /// The `predict` function returns a vector of predicted values calculated using the linear equation `y
 /// = slope * x + intercept` for each element in the input vector `x`.
    pub fn predict(x: Vec<f64>, slope: f64, intercept: f64) -> Vec<f64> {
        x.iter().map(|xi| ((xi * slope) + intercept)).collect()
    }

/// The function calculates the mean squared error between two vectors of floating-point numbers.
/// 
/// Arguments:
/// 
/// * `y_true`: The `y_true` parameter represents a vector of actual (true) values. It contains the true
/// values of the target variable for a set of data points.
/// * `y_pred`: A vector containing the predicted values.
/// 
/// Returns:
/// 
/// The `mean_squared_error` function calculates the mean squared error between two vectors `y_true` and
/// `y_pred`. It returns a single `f64` value representing the mean squared error.
    pub fn mean_squared_error(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
        y_true
            .iter()
            .zip(y_pred.iter())
            .map(|(yt, yp)| ((yt - yp).powi(2)))
            .sum::<f64>()
            / y_true.len() as f64
    }
}



pub struct MultipleLinearRegression;

impl MultipleLinearRegression{
    pub fn transpose() {
        
    }
    pub fn matrix_multiplication(){

    }
    pub fn invert_matrix(){

    }
    pub fn train(){

    }
    pub fn predict(){

    }
}