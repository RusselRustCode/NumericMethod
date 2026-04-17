use std::{f64::consts::LN_2, vec};

fn new_fi_func(x: &f64, lambda: &f64) -> f64 {
    return x - lambda * func(&x);
}

fn simple_iter(a: &f64, b: &f64, eps: &f64, lambda: &f64) -> (f64, i32) {
    let mut count = 0;
    let mut x: f64;
    let mut x0 = *a;
    loop {
        x = new_fi_func(&x0, lambda);
        count += 1;
        if (x - x0).abs() < *eps {
            return (x, count);
        }
        x0 = x;
    }
}

fn func(x: &f64) -> f64 {
    return 2.0_f64.powf(*x) + x.powi(2) / 2.0 - 4.0;
} //главная функция

fn derivation(x: &f64) -> f64 {
    return 2.0_f64.powf(*x) * LN_2 + x;
}

fn fi_func(x: &f64) -> f64 {
    return (8.0 - 2.0 * 2.0_f64.powf(*x)).sqrt();
    //нерабочая функция
}

fn fi_derivation(x: &f64) -> f64 {
    // return 2.0_f64.powf(*x) * LN_2 / fi_func(x);
    return 2.0_f64.powf(*x) * LN_2 + x + 1.0;
}

fn simple_iter1(a: &f64, b: &f64, eps: &f64, max_iter: i32, lambda: &f64) -> (f64, i32) {
    let mut count = 0;
    let mut x: f64;
    let mut x0 = *a;

    while count < max_iter {
        x = fi_func3(&x0);
        count += 1;

        if (x - x0).abs() < *eps {
            return (x, count);
        }

        x0 = x;
    }

    // Если достигнуто максимальное количество итераций, возвращаем последнее значение x
    (x0, count)
}

fn fi_func2(x: &f64) -> f64 {
    return (2.0_f64.powf(*x) + x.powf(2.0) / 2.0 - 4.0 + x);
} //нерабочая функция

fn fi_func3(x: &f64) -> f64 {
    return (4.0_f64 - x.powf(2.0) / 2.0).log2();
}

fn fi_deriv_func3(x: &f64) -> f64 {
    return -(x / (4.0 - x.powf(2.0) / 2.0) * LN_2);
}

