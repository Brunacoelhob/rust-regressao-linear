/// Calcula os coeficientes da regressão linear (intercepto e inclinação).
/// Retorna uma tupla (intercepto, inclinação).
pub fn linear_regression(x: &[f64], y: &[f64]) -> Result<(f64, f64), &'static str> {
    if x.len() != y.len() || x.is_empty() {
        return Err("Os vetores x e y devem ter o mesmo tamanho e não podem estar vazios.");
    }

    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
    let sum_x_squared: f64 = x.iter().map(|xi| xi * xi).sum();

    let denominator = n * sum_x_squared - sum_x * sum_x;
    if denominator == 0.0 {
        return Err("Divisão por zero ao calcular os coeficientes.");
    }

    let slope = (n * sum_xy - sum_x * sum_y) / denominator;
    let intercept = (sum_y - slope * sum_x) / n;

    Ok((intercept, slope))
}

/// Calcula o coeficiente de determinação (R²) para a regressão linear.
pub fn r_squared(x: &[f64], y: &[f64], intercept: f64, slope: f64) -> Result<f64, &'static str> {
    if x.len() != y.len() || x.is_empty() {
        return Err("Os vetores x e y devem ter o mesmo tamanho e não podem estar vazios.");
    }

    let mean_y: f64 = y.iter().sum::<f64>() / y.len() as f64;
    let ss_total: f64 = y.iter().map(|yi| (yi - mean_y).powi(2)).sum();
    let ss_residual: f64 = x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (yi - (intercept + slope * xi)).powi(2))
        .sum();

    Ok(1.0 - (ss_residual / ss_total))
}

/// Calcula o erro quadrático médio (MSE) para a regressão linear.
pub fn mean_squared_error(x: &[f64], y: &[f64], intercept: f64, slope: f64) -> Result<f64, &'static str> {
    if x.len() != y.len() || x.is_empty() {
        return Err("Os vetores x e y devem ter o mesmo tamanho e não podem estar vazios.");
    }

    let mse: f64 = x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (yi - (intercept + slope * xi)).powi(2))
        .sum::<f64>()
        / x.len() as f64;

    Ok(mse)
}

/// Realiza previsões com base nos coeficientes da regressão linear.
/// Retorna um vetor com os valores previstos.
pub fn predict(x: &[f64], intercept: f64, slope: f64) -> Vec<f64> {
    x.iter().map(|&xi| intercept + slope * xi).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_regression() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = linear_regression(&x, &y).unwrap();
        assert!((result.0 - 0.0).abs() < 1e-6); // Intercepto deve ser 0
        assert!((result.1 - 2.0).abs() < 1e-6); // Inclinação deve ser 2
    }

    #[test]
    fn test_invalid_input() {
        let x = vec![1.0, 2.0];
        let y = vec![3.0];
        assert!(linear_regression(&x, &y).is_err());
    }

    #[test]
    fn test_r_squared() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let (intercept, slope) = linear_regression(&x, &y).unwrap();
        let r2 = r_squared(&x, &y, intercept, slope).unwrap();
        assert!((r2 - 1.0).abs() < 1e-6); // R² deve ser 1.0 para dados perfeitamente lineares
    }

    #[test]
    fn test_mean_squared_error() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let (intercept, slope) = linear_regression(&x, &y).unwrap();
        let mse = mean_squared_error(&x, &y, intercept, slope).unwrap();
        assert!((mse - 0.0).abs() < 1e-6); // MSE deve ser 0.0 para dados perfeitamente lineares
    }

    #[test]
    fn test_predict() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let intercept = 0.0;
        let slope = 2.0;
        let predictions = predict(&x, intercept, slope);
        let expected = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        assert_eq!(predictions, expected);
    }
}