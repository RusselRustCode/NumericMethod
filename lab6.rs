use std::f64::consts::PI;

fn func(x: &f64) -> f64 {
    return x.cos();
}

fn midpoint(x1: &f64, x2: &f64, n: &f64) -> f64 {
    let delta_x = (x2 - x1) / n;
    let mut result = 0.0;
    let mut val;
    for i in 0..*n as usize {
        val = x1 + (i as f64 + 0.5) * delta_x;
        result += func(&val) * delta_x;
    }
    return result;
}

fn leftpoint(x1: &f64, x2: &f64, n: &f64) -> f64 {
    let delta_x = (x2 - x1) / n;
    let mut result = 0.0;
    let mut val;
    for i in 0..*n as usize {
        val = x1 + (i as f64 * delta_x);
        result += func(&val) * delta_x;
    }
    return result;
}

fn rightpoint(x1: &f64, x2: &f64, n: &f64) -> f64 {
    let delta_x = (x2 - x1) / n;
    let mut result = 0.0;
    let mut val;
    for i in 1..*n as usize + 1 {
        val = x1 + (i as f64 * delta_x);
        result += func(&val) * delta_x;
    }
    return result;
}

fn trapezoidal(x1: &f64, x2: &f64, n: &f64) -> f64 {
    let delta_x = (x2 - x1) / n;
    let mut result = 0.0;
    for i in 0..*n as usize {
        result += ((func(&(x1 + i as f64 * delta_x)) + func(&(x1 + (i as f64 + 1.0) * delta_x)))
            / 2.0)
            * delta_x
    }
    return result;
}

fn simpson(x1: &f64, x2: &f64, n: &f64) -> f64 {
    let delta_x = (x2 - x1) / n;
    let mut result = 0.0;
    for i in 0..*n as usize {
        result += (func(&(x1 + i as f64 * delta_x))
            + 4.0 * func(&(x1 + (i as f64 + 0.5) * delta_x))
            + func(&(x1 + (i as f64 + 1.0) * delta_x)))
            * delta_x
            / 6.0;
    }
    return result;
}

fn teor_eps_left_right_point(x1: &f64, x2: &f64, n: &f64) -> f64 {
    let max_deriv = 1.0;
    return max_deriv * ((x2 - x1).powf(2.0) / (2.0 * n));
}

fn teor_eps_midpoint(x1: &f64, x2: &f64, n: &f64) -> f64 {
    let max_deriv = 1.0;

    return max_deriv * ((x2 - x1).powf(3.0) / (24.0 * n.powf(2.0)));
}

fn teor_eps_trapeziodal(x1: &f64, x2: &f64, n: &f64) -> f64 {
    let max_deriv = 1.0;

    return max_deriv * ((x2 - x1).powf(3.0) / (12.0 * n.powf(2.0)));
}

fn teor_eps_simpson(x1: &f64, x2: &f64, n: &f64) -> f64 {
    let max_deriv = 1.0;

    return max_deriv * ((x2 - x1).powf(5.0) / (2880.0 * n.powf(4.0)));
}

fn find_n(x1: &f64, x2: &f64, eps: f64, exact: f64, method: &str) -> u32 {
    let mut n = 1;
    let max_iterations = 1000000;
    let mut integral = 0.0;

    loop {
        match method {
            "left" => integral = leftpoint(x1, x2, &(n as f64)),
            "right" => integral = rightpoint(x1, x2, &(n as f64)),
            "midpoint" => integral = midpoint(x1, x2, &(n as f64)),
            "trapezoidal" => integral = trapezoidal(x1, x2, &(n as f64)),
            "simpson" => {
                if n % 2 != 0 {
                    n += 1;
                }
                integral = simpson(x1, x2, &(n as f64));
            }
            _ => panic!("Неизвестный метод"),
        }
        if (exact - integral).abs() < eps {
            break;
        }
        if n > max_iterations {
            break;
        }
        n += 1;
    }
    return n;
}
