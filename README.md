# Regressão Linear em Rust

Este projeto implementa uma **regressão linear pura** em Rust, sem o uso de bibliotecas externas para cálculos matemáticos. Ele inclui funcionalidades para calcular os coeficientes da regressão, avaliar a qualidade do modelo e realizar previsões para novos valores.

---

## **Objetivo**
O objetivo deste projeto é fornecer uma implementação eficiente e simples de regressão linear para análise de séries temporais, com foco em aprendizado e prática de programação em Rust.

---

## **Funcionalidades**
1. **Regressão Linear**:
   - Calcula os coeficientes da reta (intercepto e inclinação) que melhor se ajusta aos dados fornecidos.

2. **Métricas de Avaliação**:
   - **R² (Coeficiente de Determinação)**: Mede a qualidade do ajuste do modelo.
   - **MSE (Erro Quadrático Médio)**: Mede o erro médio entre os valores reais e os previstos.

3. **Previsões**:
   - Realiza previsões para novos valores com base nos coeficientes calculados.

---

## **Estrutura do Projeto**

rust-linear-regression ├── src │ ├── regression │ │ ├── linear.rs # Implementação da regressão linear e métricas │ │ └── mod.rs # Módulo para expor as funções de regressão │ ├── utils │ │ └── mod.rs # Funções auxiliares │ ├── lib.rs # Biblioteca principal │ └── main.rs # Ponto de entrada do programa ├── Cargo.toml # Configuração do projeto ├── Cargo.lock # Gerado automaticamente pelo Cargo └── README.md # Documentação do projeto

---

## **Como Executar**

### **Pré-requisitos**
- Instale o Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### **Passos**
1. Clone o repositório:
   ```bash
   git clone https://github.com/Brunacoelhob/rust-regressao-linear.git
   cd rust-regressao-linear

2. Compile o projeto:
cargo build

4. Execute o programa:
cargo run

4- Execute os testes:
cargo test

