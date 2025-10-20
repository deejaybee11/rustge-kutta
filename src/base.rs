use ndarray::{ArrayD, Array2, Array1, arr1, arr2};
use num_complex::Complex64;

fn check_arguments(f: impl Fn(f64, &[Complex64]) -> Vec<Complex64>, y: &[f64], support_complex: bool) {

    if y0.ndim() == 1{
        panic!("The initial condition array 'y0' must be 1D.");
    }

    if y0.iter().any(|x| !x.is_finite()) {
        panic!("The initial condition array 'y0' contains non-finite values.");
    }
    
    fn fun_wrapped<F, R>(fun: F) -> impl Fn(f64, &Array1<Complex64>) -> Array1<Complex64>
    where
        F: Fn(f64, &Array1<Complex64>) -> R + 'static,
        R: Into<Array1<Complex64>>,
    {
        move |t, y| fun(t, y).into()
    }
    return fun_wrapped(t,y0),y0
}

struct ODESolver {

    t_old: f64,
    t: f64,
    fun: Box<dyn Fn(f64, &[f64]) -> Vec<Complex64>>,
    y: Array1<Complex64>,
    t_bound: f64,
    vectorized: bool,
    fun_single: Box<dyn Fn(f64, &[Complex64]) -> Vec<Complex64>>,
    fun_vectorized: Box<dyn Fn(f64, &Array2<Complex64>) -> Array2<Complex64>>,
    direction: i32,
    n: i32,
    status: String,
    nfev: i32,
    njev: i32,
    nlu: i32,
}

impl ODESolver {
    fn new<F>(
        fun: F,
        t0: f64,
        y0: Array1<f64>,
        t_bound: f64,
        vectorized: bool,
    ) -> Self
    where
        F: Fn(f64, &[f64]) -> Vec<Complex64> + 'static,
    {
        let support_complex = true; // Assume we want to support complex numbers
        let (fun_wrapped, y0_complex) = check_arguments(fun, &y0, support_complex);

        if vectorized {
            // Vectorized case
            let fun_single = Box::new(move |t: f64, y: &Array1<f64>| -> Array1<f64> {
                // Convert y to a 2D array with one column
                let y_col = y.clone().insert_axis(Axis(1));
                // Call vectorized function and flatten result
                fun(t, &y_col).to_owned()
            });
            
            let fun_vectorized = Box::new(move |t, y| fun(t, y));
            
            (fun_single, fun_vectorized)
        } else {
            // Non-vectorized case
            let fun_single = Box::new(fun);
            
            let fun_vectorized = Box::new(move |t: f64, y: &Array2<f64>| -> Array2<f64> {
                let n_rows = y.nrows();
                let n_cols = y.ncols();
                let mut f = Array2::zeros((n_rows, n_cols));
                
                // Apply function column by column
                for i in 0..n_cols {
                    let yi = y.column(i).to_owned();
                    let fi = fun(t, &yi);
                    f.column_mut(i).assign(&fi);
                }
                f
            });
            
            (fun_single, fun_vectorized)
        }

        fn fun<F>(t: f64, y: &Array1<Complex64>) -> Array1<Complex64>
        where
            F: Fn(f64, &Array1<Complex64>) -> Array1<Complex64>,
        {
            self.nfev += 1;
            fun_single(t, y)
        }
        
        let direction = if t_bound >= t0 { 1 } else { -1 };

        ODESolver {
            t_old: t0,
            t: t0,
            fun: Box::new(fun_wrapped),
            y: y0_complex,
            t_bound,
            vectorized,
            fun_single: Box::new(fun_wrapped),
            fun_vectorized: Box::new(|t, y| {
                // Placeholder for vectorized function
                Array2::<Complex64>::zeros((y.nrows(), y.ncols()))
            }),
            direction,
            n: y0.len() as i32,
            status: "running".to_string(),
            nfev: 0,
            njev: 0,
            nlu: 0,
        }
    }

    fn step_size(&self) -> f64 {
        if self.t_old is None {
            return None;
        } else {
            return (self.t - self.t_old).abs();
        }
    }

    fn step(&mut self) -> String{
        let mut message = None;
        let mut succecss = None;
        if self.status != "running" {
            panic!("Cannot call step() when the solver is not running.");
        }
        if self.n == 0 || self.t == self.t_bound {
            self.t_old = self.t;
            self.t = self.t_bound;
            message = None;
            self.status = "finished".to_string();
        } else {
            t = self.t;
            success, message = self.step_impl();
            if !success {
                self.status = "failed".to_string();
            } else {
                self.t_old = self.t;
                if self.direction * (self.t - self.t_bound) >= 0.0 {
                    self.status = "finished".to_string();
                }
            }
        }
        return message
    }

    fn dense_output(&self) -> Array1<Complex64> {
        if self
}