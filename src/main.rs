mod s_y;

fn main() {

    let mut s: s_y::ShuntingYard = s_y::ShuntingYard::new();
    let r: Result<f64, Vec<String>> = s.calculate("2 + 2");
    println!("{:?}", r);
    assert_eq!(r.unwrap(), 4.0);

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

