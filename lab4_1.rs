fn cubic_spline_func(xvalues: &[f64], yvalues: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = xvalues.len() - 1; // Количество интервалов, а не точек
    let mut x_del: Vec<f64> = vec![0.0; n];
    let mut coeff = vec![0.0; n];
    let mut sec_deriv = vec![0.0; n + 1];

    for i in 0..n {
        x_del[i] = xvalues[i + 1] - xvalues[i];
        coeff[i] = (yvalues[i + 1] - yvalues[i]) / x_del[i];
    }

    sec_deriv[0] = 0.0;
    for i in 0..n - 1 {
        sec_deriv[i + 1] = 3.0 * (coeff[i + 1] - coeff[i]) / (x_del[i + 1] + x_del[i]);
    }
    sec_deriv[n] = 0.0;

    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    let mut c = vec![0.0; n];
    let mut d = vec![0.0; n];

    for i in 0..n {
        a[i] = (sec_deriv[i + 1] - sec_deriv[i]) / (6.0 * x_del[i]);
        b[i] = sec_deriv[i] / 2.0;
        c[i] = coeff[i] - x_del[i] * (sec_deriv[i + 1] + 2.0 * sec_deriv[i]) / 6.0;
        d[i] = yvalues[i];
    }

    (a, b, c, d)
}

fn parabolic_spline_func(xvalues: &[f64], yvalues: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = xvalues.len() - 1; // Количество интервалов, а не точек
    let mut x_del: Vec<f64> = vec![0.0; n];
    let mut first_deriv = vec![0.0; n + 1];

    // Вычисление шагов интерполяции
    for i in 0..n {
        x_del[i] = xvalues[i + 1] - xvalues[i];
    }

    // Вычисление первых производных в узлах интерполяции
    for i in 1..n {
        first_deriv[i] = (yvalues[i + 1] - yvalues[i - 1]) / (x_del[i] + x_del[i - 1]);
    }

    // Граничные условия для первых производных
    first_deriv[0] = (yvalues[1] - yvalues[0]) / x_del[0];
    first_deriv[n] = (yvalues[n] - yvalues[n - 1]) / x_del[n - 1];

    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    let mut c = vec![0.0; n];

    // Вычисление коэффициентов параболического сплайна
    for i in 0..n {
        a[i] = (first_deriv[i + 1] - first_deriv[i]) / (2.0 * x_del[i]);
        b[i] = first_deriv[i];
        c[i] = yvalues[i];
    }

    (a, b, c)
}

