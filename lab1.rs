use ndarray::{Array1, Array2};
use ndarray::prelude::*;

fn gauss_elimination(A: &mut Array2<f64>, b: &mut Array1<f64>) -> Array1<f64> {
    let n = b.len();

    // Прямой ход метода Гаусса
    for i in 0..n {
        // Поиск максимального элемента в столбце i aka выбор главного элемента
        let mut max_el = A[[i, i]].abs();
        let mut max_row = i;
        for k in (i + 1)..n {
            if A[[k, i]].abs() > max_el {
                max_el = A[[k, i]].abs();
                max_row = k;
            }
        }

        // Обмен строк max_row и i, если необходимо
        if max_row != i {
            for j in 0..n {
                A.swap((i, j), (max_row, j));
            }
            b.swap(i, max_row);
        }

        // Нормализация строки i
        for k in (i + 1)..n {
            let c = -A[[k, i]] / A[[i, i]];
            for j in i..n {
                if i == j {
                    A[[k, j]] = 0.0;
                } else {
                    A[[k, j]] += c * A[[i, j]];
                }
            }
            b[k] += c * b[i];
        }
    }
    let mut arr = b.clone();
    // Обратный ход метода Гаусса
    let mut x = Array1::zeros(n);
    for i in (0..n).rev() {
        x[i] = arr[i] / A[[i, i]];
        for k in 0..i {
            arr[k] -= A[[k, i]] * x[i];
        }
    }
    
    x
}

fn vec_nev(array:  &Array2<f64>, coeff: &Array1<f64>, x: &Array1<f64>) -> Array1<f64>{
    let new_array = array.dot(x);
    println!("{}", &new_array);
    new_array - coeff
}

fn norm(vec: &Array1<f64>) -> f64{
    let sum: f64 = vec.iter().map(|&x| x * x).sum();

    return sum.sqrt(); 
}
fn det(vec: &Array2<f64>) -> f64{
    let mut det: f64 = 1.0;
    let n = vec.shape()[0];
    for i in 0..n{
        det *= vec[[i, i]];
    }

    return det;
}

fn rev_matrix(matrix: &Array2<f64>) -> Option<Array2<f64>>{
    let n = matrix.nrows();
    assert_eq!(n, matrix.ncols(), "Матрица должна быть квадратной");

    let mut augmented = Array2::zeros((n, 2 * n));
    augmented.slice_mut(s![.., ..n]).assign(matrix);
    augmented.slice_mut(s![.., n..]).fill(0.0);
    for i in 0..n {
        augmented[[i, n + i]] = 1.0;
    }

    for i in 0..n {
        if augmented[[i, i]] == 0.0 {
            return None; 
        }

        let pivot = augmented[[i, i]];
        for j in 0..2 * n {
            augmented[[i, j]] /= pivot; //делим на i строку, чтобы обнулить диагональный элемент 
        }

        for k in 0..n { //обнуляем столбец под диагональным элементом 
            if k != i {
                let factor = augmented[[k, i]];
                for j in 0..2 * n {
                    augmented[[k, j]] -= factor * augmented[[i, j]];
                }
            }
        }
    }

    let mut inverse = Array2::zeros((n, n));
    inverse.assign(&augmented.slice(s![.., n..]));
    Some(inverse)

}

fn matrix_cond_number(array: &Array2<f64>, rev_array: &Array2<f64>) -> f64{

    let numb1 = array.axis_iter(ndarray::Axis(0))
    .map(|row| row.fold(0.0, |acc, &x| acc + x.abs()))
    .fold(0.0, f64::max);

    let numb2 = rev_array
    .axis_iter(ndarray::Axis(0))
    .map(|row| row.fold(0.0, |acc, &x| acc + x.abs()))
    .fold(0.0, f64::max);

    return numb1*numb2;
}
