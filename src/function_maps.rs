// math expression functions

// u + v
pub fn _add (u_vector: Vec<f32>, v_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().zip(v_vector.into_iter()).map(|(u, v)| u + v ).collect();
    result_vector
}

// u - v
pub fn _subtract(u_vector: Vec<f32>, v_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().zip(v_vector.into_iter()).map(|(u, v)| u - v ).collect();
    result_vector
}

// u * v
pub fn _multiply(u_vector: Vec<f32>, v_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().zip(v_vector.into_iter()).map(|(u, v)| u * v ).collect();
    result_vector
}

// u / v
pub fn _divide(u_vector: Vec<f32>, v_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().zip(v_vector.into_iter()).map(|(u, v)| u / v ).collect();
    result_vector
}

// n * u  (n = constant)
pub fn _n_multiply(u_vector: Vec<f32>, c: f32) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u * c ).collect();
    result_vector
}

// u ^ v
pub fn _power(u_vector: Vec<f32>, v_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().zip(v_vector.into_iter()).map(|(u, v)| f32::powf(u, v)).collect();
    result_vector
}

// n ^ u  (n = constant)
pub fn _n_power_u(u_vector: Vec<f32>, n: f32) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| f32::powf(n, u)).collect();
    result_vector
}

// u ^ n  (n = constant)
pub fn _u_power_n(u_vector: Vec<f32>, n: f32) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| f32::powf(u, n)).collect();
    result_vector
}

// sin(u)
pub fn _sin(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.sin()).collect();
    result_vector
}

// cos(u)
pub fn _cos(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.cos()).collect();
    result_vector
}

// tan(u)
pub fn _tan(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.tan()).collect();
    result_vector
}

// csc(u)
pub fn _csc(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter()
        .map(|u| if u.sin() == 0.0 { f32::NAN } else { 1.0 / u.sin() })
        .collect();
    result_vector
}

// sec(u)
pub fn _sec(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter()
        .map(|u| if u.cos() == 0.0 { std::f32::NAN } else { 1.0 / u.cos() })
        .collect();
    result_vector
}

// cot(u)
pub fn _cot(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter()
        .map(|u| if u.tan() == 0.0 { std::f32::NAN } else { 1.0 / u.tan() })
        .collect();
    result_vector
}

// sinh(u)
pub fn _sinh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.sinh()).collect();
    result_vector
}

// cosh(u)
pub fn _cosh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.cosh()).collect();
    result_vector
}

// tanh(u)
pub fn _tanh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.tanh()).collect();
    result_vector
}

// asin(u)
pub fn _asin(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.asin()).collect();
    result_vector
}

// acos(u)
pub fn _acos(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.acos()).collect();
    result_vector
}

// atan(u)
pub fn _atan(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.atan()).collect();
    result_vector
}

// asinh(u)
pub fn _asinh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.asinh()).collect();
    result_vector
}

// acosh(u)
pub fn _acosh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.acosh()).collect();
    result_vector
}

// atanh(u)
pub fn _atanh(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.atanh()).collect();
    result_vector
}

// TODO:
//   sech(u), cosech(u), coth(u), asec(u), acosec(u), acot(u), asech(u), acosech(u), acoth(u)


// sqrt(u)
pub fn _sqrt(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector: Vec<f32> = u_vector.into_iter().map(|u| u.sqrt()).collect();
    result_vector
}

// log10(u)
pub fn _log10(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector = u_vector.into_iter().map(|u| u.log10()).collect();
    result_vector
}

// ln(u)
pub fn _ln(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector = u_vector.into_iter().map(|u| u.ln()).collect();
    result_vector
}

// abs(u)
pub fn _abs(u_vector: Vec<f32>) -> Vec<f32> {
    let result_vector = u_vector.into_iter().map(|u| u.abs()).collect();
    result_vector
}
