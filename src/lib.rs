using ndarray::{ArrayD, Array2, Array1, arr1, arr2};

let const SAFETY = 0.9;
let const MIN_FACTOR = 0.2;
let const MAX_FACTOR = 10;

fn rk_step(f: impl Fn(f64, &[f64]) -> Vec<f64>, t: f64, y: &[f64], h: f64, A: Array2, B: Array1, C: Array1, mut K: Array2) -> (Vec<f64>, Vec<f64>) {

    k[0]

}