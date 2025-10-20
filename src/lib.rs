using ndarray::{ArrayD, Array2, Array1, arr1, arr2};

let const SAFETY = 0.9;
let const MIN_FACTOR = 0.2;
let const MAX_FACTOR = 10;

fn rk_step(f: impl Fn(f64, &[f64]) -> Vec<f64>, t: f64, y: &[f64], h: f64, A: Array2, B: Array1, C: Array1, mut K: Array2) -> (Vec<f64>, Vec<f64>) {

    K[0] = f;
    for s, (a,c) in enumerate(zip(A[1:], C[1:])) {
        dy = K[:s].transpose().dot(&a[:s]) * h;
        K[s] = f(t + c * h, y + dy);
    
    let y_new = y + h * K[:-1].transpose().dot(&B);
    let f_new = f(t + h, &y_new);

    K[-1] = f_new;
    return (y_new, f_new);
}

pub struct RungeKutta {
    // Butcher tableau coefficients
    C: Array1<f64>,  // nodes
    A: Array2<f64>,  // Runge-Kutta matrix
    B: Array1<f64>,  // weights
    E: Array1<f64>,  // error weights
    P: Array2<f64>,  // dense output coefficients

    // Method characteristics
    order: i32,
    error_estimator_order: i32, 
    n_stages: usize,

    // State variables
    y_old: Option<Array1<f64>>,
    max_step: f64,
    rtol: f64,
    atol: f64,
    f: Array1<f64>,
    h_abs: f64,
    k: Array2<f64>,
    error_exponent: f64,
    h_previous: Option<f64>,
    n: i32,
    // Problem definition
    t: f64,
    y: Array1<f64>,
    t_bound: f64,
    direction: f64,
}

impl RungeKutta {
    pub fn new<F>(
        fun: F,
        t0: f64, 
        y0: Array1<f64>,
        t_bound: f64,
        max_step: Option<f64>,
        rtol: Option<f64>,
        atol: Option<f64>,
        vectorized: bool,
        first_step: Option<f64>,        
    ) -> Self 
    where
        F: Fn(f64, &Array1<f64>) -> Array1<f64>
    {
        let max_step = validate_max_step(max_step);
        let y_old = None;
        let rtol, atol = validate_tolerances(rtol, atol, n);


    }}