use std::ffi::{CString, CStr};
use std::os::raw::{c_char};

mod s_y;

#[cfg(test)] mod tests;


fn main() {
    let mut s: s_y::ShuntingYard = s_y::ShuntingYard::new();
    assert_eq!(s.calculate("2 + 3").unwrap(), 5.0);    
}

#[no_mangle]
pub extern "C" fn calculate_for_graph(expression_input: *const c_char, some_x: f64) -> f64 {
    let input_c_str: &CStr = unsafe { CStr::from_ptr(expression_input)};
    let expression: String = input_c_str.to_str().unwrap().to_string(); 
    let expression = expression.replace("x", &some_x.to_string());
    let result: f64 = infix_calculator(expression.to_string());
    result
}

#[no_mangle]
pub extern "C" fn calculate(input: *const c_char) -> f64 {
    let input_c_str: &CStr = unsafe { CStr::from_ptr(input)};
    let expression: String = input_c_str.to_str().unwrap().to_string(); 
    let result: f64 = infix_calculator(expression.to_string());
    result
}

pub extern "C" fn calculate_string(input: *const c_char) -> *mut c_char {
    let input_c_str: &CStr = unsafe { CStr::from_ptr(input)};
    let expression: String = input_c_str.to_str().unwrap().to_string(); 
    let result: f64 = infix_calculator(expression.to_string());
    let output = CString::new(result.to_string());
    output.unwrap().into_raw()
    //result
}

// changed xsquared to func_of_x, add expression to arguments
#[no_mangle]
pub unsafe extern "C" fn coord_vector_maker (input: *const c_char, x_lower: f64, x_upper: f64, y_lower: f64, y_upper: f64, x_precision: f64, y_precision: f64) ->  *mut CoordPair<f64> {
    let input_c_str: &CStr = CStr::from_ptr(input);
    let str_slice: &str = input_c_str.to_str().unwrap();
    let expression: String = str_slice.to_owned(); 
    
    let x_all : Vec<f64> = x_vector_maker(x_lower, x_upper, x_precision);
    let y_all : Vec<f64> = round_y_to_precision(func_of_x(expression, x_lower, x_upper, x_precision, y_lower, y_upper), y_precision);
    let x_ptr = x_all.as_ptr();
    let y_ptr = y_all.as_ptr();
    let c_out = CoordPair::new(x_ptr, y_ptr);
    
    Box::into_raw(Box::new(c_out))
}

// calculates a f64 result from a string expression, returns NaN if runtime errors occur
pub fn infix_calculator(expression: String) -> f64 {
    let mut s: s_y::ShuntingYard = s_y::ShuntingYard::new();
    assert_eq!(s.calculate("2 + 3").unwrap(), 5.0);
    let r: Result<f64, Vec<String>> = s.calculate(&expression);
    match r {
        Ok(result) => result,
        Err(e) => {
            println!("Errors: {:?}", e);
            f64::NAN
        }
    }
}

pub fn make_all_x_strings (expression: String, x_lower: f64, x_upper: f64, x_precision: f64) -> Vec<String> {
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

impl CoordPair<f64> {
    fn new(x_ptr: *const f64, y_ptr: *const f64) -> CoordPair<f64> {
        CoordPair {x_ptr: x_ptr, y_ptr: y_ptr}
    }
}

#[no_mangle]
pub fn func_of_x (expression: String, x_lower: f64, x_upper: f64, x_precision: f64, y_lower: f64, y_upper: f64) -> Vec<f64> {
    let ys: Vec<f64> = make_all_x_strings(expression, x_lower, x_upper, x_precision).into_iter().map(|expr| infix_calculator(expr) as f64).collect();
    ys //.into_iter().map(|y| if y >= y_lower && y <= y_upper  { y } else { std::f64::NAN } ).collect()
}

fn round_y_to_precision(y_all: Vec<f64>, y_precision: f64) -> Vec<f64> {
    y_all.into_iter().map(|y| round_to_precision(y, y_precision)).collect()
}

fn round_to_precision(x: f64, precision: f64) -> f64 {
    let precision_inverse: f64 = 1.0/ precision;
    (x * precision_inverse).round() / precision_inverse
}
#[no_mangle]
pub fn x_vector_maker (x_lower: f64, x_upper: f64, x_precision: f64) -> Vec<f64> {
    let mut x: f64 = x_lower;
    let x_bound_bredth: f64 = &x_upper.ceil() - &x_lower.floor(); //should be checked that lower is less than upper in frontend?
    let capacity: usize = (x_bound_bredth * (1.0 / &x_precision) ).ceil() as usize;
    let mut x_vector: Vec<f64> = Vec::with_capacity(capacity);
    while x <= x_upper { 
        x_vector.push(x);
        x = x + x_precision;
    }
    x_vector.shrink_to_fit();
    x_vector
}
