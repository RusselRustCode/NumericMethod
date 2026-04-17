#![warn(clippy::all, clippy::pedantic)]


use std::{vec};

// use ndarray::{Array1, Array2};

fn seidel(a: &Vec<Vec<f64>>, y: &Vec<f64>, n: usize, epsilon: f64) -> (Vec<f64>, i32) {
    let mut k = 0;
    let mut res = vec![0.0; n];
    let eps = epsilon;
    let mut xn = vec![0.0; n];

    for i in 0..n {
        res[i] = y[i] / a[i][i];
    }

    loop {
        for i in 0..n {
            xn[i] = y[i] / a[i][i];
            for j in 0..n {
                if i != j {
                    if j < i {
                        xn[i] -= a[i][j] / a[i][i] * xn[j];
                        k += 1;
                    } else {
                        xn[i] -= a[i][j] / a[i][i] * res[j];
                        k += 1;
                    }
                }
            }
        }

        let mut flag = true;
        for i in 0..(n - 1) {
            if (xn[i] - res[i]).abs() > eps {
                flag = false;
                break;
            }
        }

        for i in 0..n {
            res[i] = xn[i];
        }

        if flag {
            break;
        }
    }

    (res, k)
}

fn simple_iter(a: &Vec<Vec<f64>>, y: &Vec<f64>, n: usize, epsilon: f64) -> (Vec<f64>, i32){
    let mut k = 0;
    let mut res = vec![0.0; n];
    let eps = epsilon;
    let mut xn = vec![0.0; n];

    for i in 0..n {
        res[i] = y[i] / a[i][i];
    }

    loop {
        for i in 0..n {
            xn[i] = y[i] / a[i][i];
            for j in 0..n {
                if i != j {
                    xn[i] -= a[i][j] / a[i][i] * res[j]; 
                    k += 1;
                }
            }
        }
         
        let mut flag = true;
        for i in 0..(n - 1) {
            if (xn[i] - res[i]).abs() > eps {
                flag = false;
                break;
            }
        }

        for i in 0..n {
            res[i] = xn[i];
        }

        if flag {
            break;
        }
    }

    (res, k)
}
