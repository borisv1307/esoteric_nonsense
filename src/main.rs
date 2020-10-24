use std::ffi::{CString, CStr};
use std::os::raw::{c_char};

mod s_y;

#[cfg(test)] mod tests;


fn main() {
    let mut s: s_y::ShuntingYard = s_y::ShuntingYard::new();
    assert_eq!(s.calculate("2 + 3").unwrap(), 5.0);    
}

#[no_mangle]
pub extern "C" fn calculate(input: *const c_char) -> *mut c_char {
    let input_c_str: &CStr = unsafe { CStr::from_ptr(input)};
    let expression: String = input_c_str.to_str().unwrap().to_string(); 
    let result: f32 = infix_calculator(expression.to_string());
    let output = CString::new(result.to_string());
    output.unwrap().into_raw()
}

// changed xsquared to func_of_x, add expression to arguments (//TODO: strings over ffi??)
#[no_mangle]
pub unsafe extern "C" fn coord_vector_maker (input: *const c_char, x_lower: f32, x_upper: f32, y_lower: f32, y_upper: f32, x_precision: f32, y_precision: f32) ->  *mut CoordPair<f32> {
    let input_c_str: &CStr = CStr::from_ptr(input);
    let str_slice: &str = input_c_str.to_str().unwrap();
    let expression: String = str_slice.to_owned(); 
    
    let x_all : Vec<f32> = x_vector_maker(x_lower, x_upper, x_precision);
    let y_all : Vec<f32> = round_y_to_precision(func_of_x(expression, x_lower, x_upper, x_precision, y_lower, y_upper), y_precision);
    let x_ptr = x_all.as_ptr();
    let y_ptr = y_all.as_ptr();
    let c_out = CoordPair::new(x_ptr, y_ptr);
    
    Box::into_raw(Box::new(c_out))
}

pub fn infix_calculator(expression: String) -> f32 {
    let mut s: s_y::ShuntingYard = s_y::ShuntingYard::new();
    assert_eq!(s.calculate("2 + 3").unwrap(), 5.0);
    let r: Result<f32, Vec<String>> = s.calculate(&expression);
    r.unwrap() 
} 

pub fn make_all_x_strings (expression: String, x_lower: f32, x_upper: f32, x_precision: f32) -> Vec<String> {
    let x_fl_vector = x_vector_maker(x_lower, x_upper, x_precision);
    let x_string_vector: Vec<String> = x_fl_vector.into_iter().map(|x| expression.replace("x", &x.to_string())).collect();
    x_string_vector
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
pub fn func_of_x (expression: String, x_lower: f32, x_upper: f32, x_precision: f32, y_lower: f32, y_upper: f32) -> Vec<f32> {
    let ys: Vec<f32> = make_all_x_strings(expression, x_lower, x_upper, x_precision).into_iter().map(|expr| infix_calculator(expr) as f32).collect();
    ys //.into_iter().map(|y| if y >= y_lower && y <= y_upper  { y } else { std::f32::NAN } ).collect()
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
    while x <= x_upper { 
        x_vector.push(x);
        x = x + x_precision;
    }
    x_vector.shrink_to_fit();
    x_vector
}
