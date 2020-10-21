#[cfg(test)] mod tests;

// math expression functions

// u + v
fn add (u_vector: Vec<f32>, v_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().zip(v_vector.into_iter()).map(|(u, v)| u + v ).collect();
    result_vector
}

// u - v
fn subtract(u_vector: Vec<f32>, v_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().zip(v_vector.into_iter()).map(|(u, v)| u - v ).collect();
    result_vector
}

// u * v
fn multiply(u_vector: Vec<f32>, v_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().zip(v_vector.into_iter()).map(|(u, v)| u * v ).collect();
    result_vector
}

// u / v
fn divide(u_vector: Vec<f32>, v_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().zip(v_vector.into_iter()).map(|(u, v)| u / v ).collect();
    result_vector
}

// n * u  (n = constant)
fn n_multiply(u_vector: Vec<f32>, c: f32) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u * c ).collect();
    result_vector
}

// u ^ v
fn power(u_vector: Vec<f32>, v_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().zip(v_vector.into_iter()).map(|(u, v)| f32::powf(u, v)).collect();
    result_vector
}

// n ^ u  (n = constant)
fn n_power_u(u_vector: Vec<f32>, n: f32) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| f32::powf(n, u)).collect();
    result_vector
}

// u ^ n  (n = constant)
fn u_power_n(u_vector: Vec<f32>, n: f32) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| f32::powf(u, n)).collect();
    result_vector
}

// sin(u)
fn sin(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.sin()).collect();
    result_vector
}

// cos(u)
fn cos(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.cos()).collect();
    result_vector
}

// tan(u)
fn tan(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.tan()).collect();
    result_vector
}

// csc(u)
fn csc(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter()
        .map(|u| if u.sin() == 0.0 { f32::NAN } else { 1.0 / u.sin() })
        .collect();
    result_vector
}

// sec(u)
fn sec(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter()
        .map(|u| if u.cos() == 0.0 { std::f32::NAN } else { 1.0 / u.cos() })
        .collect();
    result_vector
}

// cot(u)
fn cot(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter()
        .map(|u| if u.tan() == 0.0 { std::f32::NAN } else { 1.0 / u.tan() })
        .collect();
    result_vector
}

// sinh(u)
fn sinh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.sinh()).collect();
    result_vector
}

// cosh(u)
fn cosh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.cosh()).collect();
    result_vector
}

// tanh(u)
fn tanh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.tanh()).collect();
    result_vector
}

// asin(u)
fn asin(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.asin()).collect();
    result_vector
}

// acos(u)
fn acos(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.acos()).collect();
    result_vector
}

// atan(u)
fn atan(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.atan()).collect();
    result_vector
}

// asinh(u)
fn asinh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.asinh()).collect();
    result_vector
}

// acosh(u)
fn acosh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.acosh()).collect();
    result_vector
}

// atanh(u)
fn atanh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.atanh()).collect();
    result_vector
}

// TODO:
//   sech(u), cosech(u), coth(u), asec(u), acosec(u), acot(u), asech(u), acosech(u), acoth(u)


// sqrt(u)
fn sqrt(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.sqrt()).collect();
    result_vector
}

// log10(u)
fn log10(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector = u_vector.into_iter().map(|u| u.log10()).collect();
    result_vector
}

// ln(u)
fn ln(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector = u_vector.into_iter().map(|u| u.ln()).collect();
    result_vector
}

// abs(u)
fn abs(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector = u_vector.into_iter().map(|u| u.abs()).collect();
    result_vector
}
