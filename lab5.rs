#![warn(clippy::all, clippy::pedantic)]
fn func(x: &f64) -> f64 {
    return x.sin();
}

fn make_net(n: i32, step: &f64, net: &mut Vec<f64>, a: &f64, func_values: &mut Vec<f64>) {
    let mut x0 = *a;
    for i in 0..n {
        x0 += (i as f64) * step;
        net.push(x0);
    }

    for item in net.iter() {
        func_values.push(func(item));
    }
}

fn least_squares_fit(x_values: &[f64], y_values: &[f64], m: usize) -> Vec<f64> {
    let n = x_values.len();
    let mut a = vec![vec![0.0; m + 1]; m + 1];
    let mut b = vec![0.0; m + 1];

    for i in 0..=m {
        for j in 0..=m {
            let mut sum = 0.0;
            for k in 0..n {
                sum += x_values[k].powi((i + j) as i32);
            }
            a[i][j] = sum;
        }

        let mut sum = 0.0;
        for k in 0..n {
            sum += y_values[k] * x_values[k].powi(i as i32);
        }
        b[i] = sum;
    }

    gauss_elimination(&mut a, &mut b);
    b
}

fn gauss_elimination(a: &mut [Vec<f64>], b: &mut [f64]) {
    let n = b.len();

    for i in 0..n {
        // Поиск максимального элемента в столбце
        let mut max_row = i;
        for j in (i + 1)..n {
            if a[j][i].abs() > a[max_row][i].abs() {
                max_row = j;
            }
        }

        // Обмен строк
        a.swap(i, max_row);
        b.swap(i, max_row);

        // Нормализация строки
        let pivot = a[i][i];
        for j in i..n {
            a[i][j] /= pivot;
        }
        b[i] /= pivot;

        // Исключение столбца
        for j in 0..n {
            if j != i {
                let factor = a[j][i];
                for k in i..n {
                    a[j][k] -= factor * a[i][k];
                }
                b[j] -= factor * b[i];
            }
        }
    }
}

fn mean_squared_error(x_values: &[f64], y_values: &[f64], poly: &dyn Fn(f64) -> f64) -> f64 {
    let n = x_values.len();
    let mut sum = 0.0;
    for i in 0..n {
        let error = y_values[i] - poly(x_values[i]);
        sum += error.powf(2.0);
    }
    sum / n as f64
}

fn mean_squared_error_n(
    xvalues: &[f64],
    yvalues: &[f64],
    poly: &dyn Fn(f64, u32) -> f64,
    p: u32,
) -> f64 {
    let n = xvalues.len();
    let mut sum = 0.0;
    for i in 0..n {
        let error = yvalues[i] - poly(xvalues[i], p as u32);
        sum += error.powf(2.0);
    }
    return sum / n as f64;
}

fn least_squares_linear(xvalues: &[f64], yvalues: &[f64]) -> (f64, f64) {
    let mut x_avg = 0.0;
    let mut y_avg = 0.0;
    let mut xy_avg = 0.0;
    let mut xx_avg = 0.0;
    let n = xvalues.len() as f64;
    for i in 0..xvalues.len() {
        x_avg += xvalues[i];
        y_avg += yvalues[i];
        xy_avg += xvalues[i] * yvalues[i];
        xx_avg += xvalues[i].powf(2.0);
    }
    x_avg = x_avg / n;
    y_avg = y_avg / n;
    xy_avg = xy_avg / n;
    xx_avg = xx_avg / n;

    let dx = xx_avg - x_avg.powf(2.0);

    let a = (xy_avg - x_avg * y_avg) / dx;
    let b = y_avg - a * x_avg;

    return (a, b);
}
