#[cfg(test)] mod tests;


fn main() {
    let a = 1;
    assert_eq!(a, 1);

}


#[repr(C)]
#[derive(Debug)]
pub struct CoordPair<T> {
    x_ptr: *const T,
    y_ptr: *const T,
}

impl CoordPair<f32> {
    fn new(x_ptr: *const f32, y_ptr: *const f32) -> CoordPair<f32> {
        CoordPair {x_ptr: x_ptr, y_ptr: y_ptr}
    }
}



#[no_mangle]
pub fn x_squared (x_vector: Vec<f32>, lower: f32, upper: f32) -> Vec<f32> {
    let y_vector: Vec<f32> = x_vector.into_iter().map(|x| x * x).collect();
    y_vector.into_iter().map(|y| if y >= lower && y <= upper  { y } else { std::f32::NAN } ).collect()
}

#[no_mangle]
pub unsafe extern "C" fn coord_vector_maker (x_lower: f32, x_upper: f32, y_lower: f32, y_upper: f32, x_precision: f32, y_precision: f32) ->  *mut CoordPair<f32> {
    let x_all : Vec<f32> = x_vector_maker(x_lower, x_upper, x_precision);
    let y_all : Vec<f32> = round_y_to_precision(x_squared(x_vector_maker(x_lower, x_upper, x_precision), y_lower, y_upper), y_precision);
    let x_ptr = x_all.as_ptr();
    let y_ptr = y_all.as_ptr();
    let c_out = CoordPair::new(x_ptr, y_ptr);
    
    Box::into_raw(Box::new(c_out))
}

fn round_y_to_precision(y_all: Vec<f32>, y_precision: f32) -> Vec<f32> {
    y_all.into_iter().map(|y| round_to_precision(y, y_precision)).collect()
}

fn round_to_precision(x: f32, precision: f32) -> f32 {
    let precision_inverse: f32 = 1.0/ precision;
    (x * precision_inverse).round() / precision_inverse
}
#[no_mangle]
pub fn x_vector_maker (x_lower: f32, x_upper: f32, x_precision: f32) -> Vec<f32> {
    let mut x: f32 = x_lower;
    let x_bound_bredth: f32 = &x_upper.ceil() - &x_lower.floor(); //should be checked that lower is less than upper in frontend?
    let capacity: usize = (x_bound_bredth * (1.0 / &x_precision) ).ceil() as usize;
    let mut x_vector: Vec<f32> = Vec::with_capacity(capacity);
    while x <= x_upper { //TODO: dont call loops multiple times: slow = bad
        x_vector.push(x);
        x = x + x_precision;
    }
    x_vector.shrink_to_fit();
    x_vector
}


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
