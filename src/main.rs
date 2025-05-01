use rust_linear_regression::regression::linear::{linear_regression, r_squared, mean_squared_error, predict};

fn main() {
    // Dados de entrada
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    // Calcula a regressão linear
    match linear_regression(&x, &y) {
        Ok((intercept, slope)) => {
            println!("Intercepto: {}", intercept);
            println!("Inclinação: {}", slope);

            // Calcula o coeficiente de determinação (R²)
            match r_squared(&x, &y, intercept, slope) {
                Ok(r2) => println!("R²: {}", r2),
                Err(e) => println!("Erro ao calcular R²: {}", e),
            }

            // Calcula o erro quadrático médio (MSE)
            match mean_squared_error(&x, &y, intercept, slope) {
                Ok(mse) => println!("MSE: {}", mse),
                Err(e) => println!("Erro ao calcular MSE: {}", e),
            }

            // Realiza previsões para novos valores
            let new_x = vec![6.0, 7.0, 8.0];
            let predictions = predict(&new_x, intercept, slope);
            println!("Previsões para {:?}: {:?}", new_x, predictions);
        }
        Err(e) => {
            println!("Erro ao calcular a regressão linear: {}", e);
        }
    }
}

fn main() {
    example_function();
}