### 1. **Simple Linear Regression**
- **Formula**: \( y = \beta_0 + \beta_1x + \epsilon \)
- **Use Cases**:
  - Predicting relationships between two continuous variables.
  - Estimating the effect of one variable on another.
- **Real-World Example**: 
  - Predicting a house’s price (\( y \)) based on its size (\( x \)).
- **Complex Scenario**: 
  - Estimating the effect of temperature on electricity consumption in different climates.

---

### 2. **Multiple Linear Regression**
- **Formula**: \( y = \beta_0 + \beta_1x_1 + \beta_2x_2 + \ldots + \beta_nx_n + \epsilon \)
- **Use Cases**:
  - Predicting an outcome based on multiple factors.
- **Real-World Example**: 
  - Predicting car mileage using variables like weight, engine size, and horsepower.
- **Complex Scenario**: 
  - Analyzing the impact of advertising spend across TV, radio, and newspapers on sales.

---

### 3. **Polynomial Regression**
- **Formula**: \( y = \beta_0 + \beta_1x + \beta_2x^2 + \ldots + \beta_nx^n + \epsilon \)
- **Use Cases**:
  - Modeling non-linear relationships.
- **Real-World Example**: 
  - Predicting the growth of bacteria over time (non-linear growth).
- **Complex Scenario**: 
  - Capturing a company’s revenue curve influenced by seasonal changes and market cycles.

---

### 4. **Ridge Regression**
- **Formula**: \( \text{Loss} = \text{MSE} + \lambda \sum \beta_i^2 \)
- **Use Cases**:
  - Solving multicollinearity by shrinking coefficients.
- **Real-World Example**: 
  - Predicting stock prices when independent variables are correlated.
- **Complex Scenario**: 
  - Modeling real estate prices across regions where features like location, size, and amenities are interdependent.

---

### 5. **Lasso Regression**
- **Formula**: \( \text{Loss} = \text{MSE} + \lambda \sum |\beta_i| \)
- **Use Cases**:
  - Feature selection by shrinking some coefficients to zero.
- **Real-World Example**: 
  - Selecting key factors influencing heart disease from a large dataset.
- **Complex Scenario**: 
  - Analyzing genomic data to identify significant genes affecting a disease.

---

### 6. **Elastic Net Regression**
- **Formula**: \( \text{Loss} = \text{MSE} + \lambda_1 \sum |\beta_i| + \lambda_2 \sum \beta_i^2 \)
- **Use Cases**:
  - Combines Lasso and Ridge for high-dimensional data.
- **Real-World Example**: 
  - Identifying customer churn predictors in telecom datasets.
- **Complex Scenario**: 
  - Handling multicollinear variables in high-dimensional marketing data.

---

### 7. **Logistic Regression**
- **Formula**: \( P(y=1|x) = \frac{1}{1 + e^{-(\beta_0 + \beta_1x)}} \)
- **Use Cases**:
  - Binary or categorical classification.
- **Real-World Example**: 
  - Predicting whether a customer will buy a product or not.
- **Complex Scenario**: 
  - Fraud detection in transactions by classifying them as legitimate or fraudulent.

---

### 8. **Stepwise Regression**
- **Formula**: Iterative model selection (forward or backward).
- **Use Cases**:
  - Simplifying models by retaining significant predictors.
- **Real-World Example**: 
  - Predicting housing prices while selecting key features automatically.
- **Complex Scenario**: 
  - Optimizing an economic model with hundreds of predictors while ensuring interpretability.

---

### 9. **Quantile Regression**
- **Formula**: Minimizes weighted residuals for specific quantiles.
- **Use Cases**:
  - Modeling medians or other quantiles instead of the mean.
- **Real-World Example**: 
  - Estimating the 90th percentile of travel times in traffic systems.
- **Complex Scenario**: 
  - Modeling housing prices where outliers heavily skew the data.

---

### 10. **Bayesian Linear Regression**
- **Formula**: Probabilistic estimation of coefficients.
- **Use Cases**:
  - Provides uncertainty quantification for predictions.
- **Real-World Example**: 
  - Predicting outcomes in clinical trials with limited data.
- **Complex Scenario**: 
  - Estimating demand in markets with volatile consumer behavior and high uncertainty.

---

### 11. **Robust Regression**
- **Formula**: Modifies the loss function to reduce outlier influence.
- **Use Cases**:
  - Handling datasets with outliers.
- **Real-World Example**: 
  - Estimating median income in a population with income inequality.
- **Complex Scenario**: 
  - Predicting real estate prices in areas with extreme outliers like mansions.

---

### 12. **Partial Least Squares (PLS) Regression**
- **Formula**: Projects predictors to a smaller space to reduce multicollinearity.
- **Use Cases**:
  - High-dimensional and multicollinear datasets.
- **Real-World Example**: 
  - Predicting wine quality based on chemical composition.
- **Complex Scenario**: 
  - Building a predictive model for climate changes using hundreds of interrelated variables.

---

### 13. **Principal Component Regression (PCR)**
- **Formula**: Applies PCA on predictors before regression.
- **Use Cases**:
  - Dimensionality reduction with correlated predictors.
- **Real-World Example**: 
  - Predicting energy consumption in factories with dozens of sensors.
- **Complex Scenario**: 
  - Modeling customer preferences in a market survey with overlapping factors.

---

### 14. **Hierarchical Regression**
- **Formula**: Multilevel models considering group effects.
- **Use Cases**:
  - Analyzing data with nested structures.
- **Real-World Example**: 
  - Understanding student performance across schools and districts.
- **Complex Scenario**: 
  - Evaluating drug effectiveness across different hospitals with varying demographics.

---

### 15. **Functional Regression**
- **Formula**: Models relationships between functions or curves.
- **Use Cases**:
  - Time-series or longitudinal data.
- **Real-World Example**: 
  - Predicting blood pressure trends based on diet over time.
- **Complex Scenario**: 
  - Forecasting electricity consumption patterns over seasons.

---

### 16. **Spline Regression**
- **Formula**: Fits piecewise polynomials with continuity constraints.
- **Use Cases**:
  - Capturing non-linear relationships while ensuring smooth transitions.
- **Real-World Example**: 
  - Predicting crop yield based on temperature and rainfall data.
- **Complex Scenario**: 
  - Modeling sales data with sudden jumps due to seasonal promotions.

---

### 17. **Zero-Inflated Regression**
- **Formula**: Handles excess zero outcomes in data.
- **Use Cases**:
  - Count data with many zero values.
- **Real-World Example**: 
  - Modeling insurance claims where many customers make no claims.
- **Complex Scenario**: 
  - Predicting city bus ridership with days of zero passengers on certain routes.

---