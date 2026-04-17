fn hord(a: &mut f64, b: &mut f64, eps: &f64) -> (f64, i32) {
    let mut count = 0;
    let mut x1 = *a;
    loop {
        let x2 = *a - (func(&a) * (*a - *b)) / (func(&a) - func(&b));
        count = count + 1;
        if (x1 - x2).abs() < *eps {
            return (x2, count);
        }
        let result: f64 = func(&x2) * func(&a);
        match result.partial_cmp(&0.0) {
            Some(std::cmp::Ordering::Greater) => *b = x2,
            Some(std::cmp::Ordering::Less) => *a = x2,
            Some(std::cmp::Ordering::Equal) => return (0.0, 0),
            None => return (0.0, 0),
        }
        x1 = x2;
    }
}

fn newthon(a: &mut f64, eps: &f64) -> (f64, i32) {
    let mut x0 = *a;
    let mut count = 0;
    loop {
        let res = x0 - func(&x0) / derivative(&x0);
        count += 1;
        if (res - x0).abs() < *eps {
            return (res, count);
        }
        x0 = res;
    }
}

fn bisekciy(a: &mut f64, b: &mut f64, eps: &f64) -> (f64, i32) {
    let mut x = 0.0;
    let mut count = 0;
    while (*b - *a) / 2.0 > *eps {
        x = (*a + *b) / 2.0;
        let result = func(&x) * func(&a);
        match result.partial_cmp(&0.0) {
            Some(std::cmp::Ordering::Greater) => *a = x,
            Some(std::cmp::Ordering::Less) => *b = x,
            Some(std::cmp::Ordering::Equal) => break,
            None => break,
        }
        count += 1;
    }
    return (x, count);
}

fn sec(a: &f64, b: &f64, eps: &f64) -> (f64, i32) {
    let mut prev = *a;
    let mut curr = *b;
    let mut count = 0;
    loop {
        let f_prev = func(&prev);
        let f_curr = func(&curr);
        count += 1;
        if f_curr.abs() < *eps {
            return (curr, count);
        }

        let next = curr - f_curr * (curr - prev) / (f_curr - f_prev);

        prev = curr;
        curr = next;
    }
}

fn func(x: &f64) -> f64 {
    return x.sinh() - x + 1.0;
}

fn derivative(x: &f64) -> f64 {
    return x.cosh() - 1.0;
}

