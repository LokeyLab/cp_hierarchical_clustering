use core::f64;
use std::error::Error;

pub(in crate::matrix_op) fn mean(x: &Vec<f64>) -> Result<f64, Box<dyn Error>> {
    let n = x.len() as f64;
    let sum: f64 = x.iter().sum();
    return Ok(sum / n);
}

pub(in crate::matrix_op) fn sum(x: &Vec<f64>) -> Result<f64, Box<dyn Error>> {
    return Ok(x.iter().sum());
}

pub(in crate::matrix_op) fn multiply(u: &[f64], v: &[f64]) -> Vec<f64> {
    return u.iter().zip(v.iter()).map(|(a, b)| a * b).collect();
}
