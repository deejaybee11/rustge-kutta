fn validate_first_step(first_step: f64, t0: f64, t_bound: f64) -> f64 {
    if first_step <= 0.0 {
        panic!("`first_step` must be positive.");
    }
    if first_step > (t_bound - t0).abs() {
        panic!("`first_step` is larger than the interval to integrate.");
    }
    return first_step;
}

fn validate_max_step(max_step: f64) -> f64 {
    if max_step <= 0.0 {
        panic!("`max_step` must be positive.");
    }
    return max_step;
}

fn warn_extraneous(extraneous) {
    if extraneous {
        eprintln!("Warning: Extraneous arguments were provided and will be ignored.");
    }
}

fn validate_tol(rtol, atol, n) -> (f64, f64) {

    if rotl.iter().any(|&x| x < 100.0 * EPS) {
        eprintln!("At least one element of `rtol` is too small. Setting rtol = maximum(rtol, {100 * EPS})");
        rtol = rtol.max(100 * EPS)
    }

    if atol.ndim() > 0 && atol.shape()[0] != n as usize {
        panic!("`atol` has incorrect shape.");
    }

    if atol.iter().any(|&x| x < 0.0) {
        panic!("All elements of `atol` must be non-negative.");
    }
    return (rtol, atol);
}

fn norm(x: Complex64) -> f64 {
    x.norm() / x.size().sqrt()
}

fn select_initial_step(F: impl Fn(f64, &Array1<f64>) -> Array1<f64>, t0: f64, y0: Array1<f64>, t_bound: f64, max_step: f64, f0: Array1<f64>, direction: f64,
    order: i32, rtol: f64, atol: f64) -> f64 {

    if y0.size() == 0 {
        return infinity;
    }
    let scale = atol + rtol * y0.abs();
    let d0 = norm(& (y0 / scale));
    let d1 = norm(& (f0 / scale));
    if d0 < 1e-5 || d1 < 1e-5 {
        let h0 = 1e-6;
    } else {
        let h0 = 0.01 * (d0 / d1);
    }

    let y1 = y0 + h0 * direction * f0;
    let f1 = F(t0 + h0 * direction, &y1);
    let d2 = norm(& ((f1 - f0) / scale)) / h0;

    if d1 <= 1e-15 && d2 <= 1e-15 {
        let h1 = max(1e-6, h0 * 1e-3);
    } else {
        let h1 = (0.01 / max(d1, d2)).powf(1.0 / (order as f64 + 1.0));
    }

    return min(100.0 * h0, h1).min(max_step);
}

