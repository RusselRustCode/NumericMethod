fn func(x: &f64) -> f64 {
    return x.sin();
}

//Нерабочая функция
// fn Langrange(net: &Vec<f64>, func_value: &Vec<f64>) -> f64{
//     let mut lang_pol = 0.0;
//     let mut vector: Vec<impl Fn(f64) -> f64> = Vec::new();
//     for i in 0..net.len(){
//         vector.push(create_basic_poly(net, &(i as i32)) as f64);
//     }
//     unimplemented!()
// }

fn newton_interpolation(x: &f64, x_values: &Vec<f64>, y_values: &Vec<f64>) -> f64 {
    let n = x_values.len();
    assert_eq!(n, y_values.len(), "Должна быть одинаковая длина векторов ");

    // Вычисляем разделенные разности
    let mut divided_differences = vec![0.0; n];
    for i in 0..n {
        divided_differences[i] = divided_diff(x_values, y_values, 0, i);
    }

    // Вычисляем значение многочлена Ньютона в точке x
    let mut result = divided_differences[0]; // f(x0)
    let mut product = 1.0;

    for i in 1..n {
        product *= x - x_values[i - 1];
        result += divided_differences[i] * product;
    }

    result
}

fn divided_diff(xvalues: &Vec<f64>, yvalues: &Vec<f64>, start: usize, end: usize) -> f64 {
    if start == end {
        return yvalues[start];
    }

    let left = divided_diff(xvalues, yvalues, start + 1, end);
    let right = divided_diff(xvalues, yvalues, start, end - 1);

    return (left - right) / (xvalues[end] - xvalues[start]);
}

//Полный провал, нерабочая функция
// fn Newton(xvalues: &Vec<f64>, yvalues: &Vec<f64>) -> impl Fn(f64) -> f64 {
//     let mut div_diff: Vec<f64> = vec![];
//     let xvalues = Rc::new(xvalues.clone());
//     let yvalues = Rc::new(yvalues.clone());
//     for i in 0..xvalues.len() {
//         div_diff.push(divided_diff(&xvalues, &yvalues, i as i32));
//     }

//     let div_diff = Rc::new(div_diff.clone());

//     let newton_poly = move |x: f64| -> f64 {
//         let mut res = *yvalues.get(0).unwrap();
//         for k in 0..yvalues.len() {
//             let mut mul = 1.0;
//             for j in 0..k {
//                 mul *= (x - xvalues.get(j).unwrap());
//             }
//             if k > 0 {
//                 res += div_diff.get(k - 1).unwrap() * mul;
//             }
//         }
//         return res;
//     };
//     return newton_poly;
// }

fn teor_epsilon(x: f64, xvalues: &Vec<f64>, yvalues: &Vec<f64>) -> f64 {
    let n = xvalues.len();
    let max_deriv = 1.0; // Максимальная производная sin(x) на интервале [-1, 1] равна 1

    let mut prod = 1.0;
    for i in 0..n {
        prod *= x - xvalues.get(i).unwrap();
    }

    return max_deriv * prod / factorial(n as u64);
}

fn factorial(n: u64) -> f64 {
    (1..=n).map(|x| x as f64).product()
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

//НЕрабочая функция, для вычисления базового полинома Лагранжа
// fn create_basic_poly<'a>(net: &'a Vec<f64>, i: &'a i32) -> impl Fn(f64) -> f64 + 'a{
//     let base = |x: f64| -> f64{
//         let mut divider = 1.0;
//         let mut result = 1.0;

//         for j in 0..net.len() as i32{
//             if j != *i{
//                 result *= (x - net[j as usize]);
//                 divider *= (net[*i as usize] - net[j as usize]);
//             }
//         }
//         return result / divider;
//     };
//     return base;
// }

fn norm_langrage(x: &f64, xvalues: &Vec<f64>, yvalues: &Vec<f64>) -> f64 {
    let mut langPol = 0.0;
    let size = xvalues.len();
    for i in 0..size {
        let mut basicsPol = 1.0;
        for j in 0..size {
            if i != j {
                basicsPol *= (x - xvalues.get(j as usize).unwrap())
                    / (xvalues.get(i as usize).unwrap() - xvalues.get(j as usize).unwrap())
            }
        }

        langPol += basicsPol * yvalues.get(i as usize).unwrap();
    }
    return langPol;
}

