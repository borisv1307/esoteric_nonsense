use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

pub fn add_matr(matr1: &Vec<Vec<f64>>, matr2: &Vec<Vec<f64>>) -> Vec<Vec<f64>>{

    let rows = matr1.len();
    let cols = matr1[0].len();

    let mut result_vec: Vec<Vec<f64>> = vec![vec![0.0; cols]; rows];

    for i in 0..(matr1.len()) {
        for a in 0..(matr1[i].len()) {
            let val = matr1[i][a] + matr2[i][a];
            result_vec[i][a] = val;
        }
    }

    return result_vec;
}

pub fn sub_matr(matr1: &Vec<Vec<f64>>, matr2: &Vec<Vec<f64>>) -> Vec<Vec<f64>>{

    let rows = matr1.len();
    let cols = matr1[0].len();

    let mut result_vec: Vec<Vec<f64>> = vec![vec![0.0; cols]; rows];

    for i in 0..(matr1.len()) {
        for a in 0..(matr1[i].len()) {
            let val = matr1[i][a] - matr2[i][a];
            result_vec[i][a] = val;
        }
    }

    return result_vec;
}

pub fn create_vec_from_string(matr_c_str: *const c_char) -> Vec<Vec<f64>>{

    let matr_c_string: &CStr = unsafe { CStr::from_ptr(matr_c_str)};
    let matrix: String = matr_c_string.to_str().unwrap().to_string();
    let row_vec: Vec<&str> = matrix.split("!").collect();

    let vec_cap = row_vec.len();
    let mut main_vec: Vec<Vec<f64>> = Vec::with_capacity(vec_cap);

    for i in 0..row_vec.len(){
        let col_vec: Vec<&str> = row_vec[i].split(",").collect();
        let mut mod_col_vec: Vec<f64> = Vec::with_capacity(col_vec.len());
        for a in 0..col_vec.len(){
            mod_col_vec.push(col_vec[a].parse().unwrap());
        }
        main_vec.push(mod_col_vec);
    }

    return main_vec;
}

pub fn create_formatted_string_from_vec(matr_vec: Vec<Vec<f64>>) -> *mut c_char {

    let mut matr_string = String::from("");

    for i in 0..(matr_vec.len()){
        for a in 0..(matr_vec[i].len()){
            let mut to_add: String = matr_vec[i][a].to_string();
            if a < (matr_vec[i].len()-1) {
                to_add = to_add + ",";
            }
            matr_string = matr_string + &to_add;
        }
        if i < (matr_vec.len()-1){
            matr_string = matr_string + "!";
        }
    }

    let c_string = CString::new(matr_string.to_string());
    let c_char_output: *mut c_char = c_string.unwrap().into_raw();
    return c_char_output;


}

pub extern fn calculate_matr(matr1_c_str: *const c_char, matr2_c_str: *const c_char, instructions: *const c_char) -> *mut c_char {
    
    let matr_vec1: Vec<Vec<f64>> = create_vec_from_string(matr1_c_str);
    let matr_vec2: Vec<Vec<f64>> = create_vec_from_string(matr2_c_str);
    let mut result_vec: Vec<Vec<f64>> = Vec::new();

    let inst_c_string: &CStr = unsafe { CStr::from_ptr(instructions)};
    let instruct_string: String = inst_c_string.to_str().unwrap().to_string();

    if instruct_string == "add"{
        result_vec = add_matr(&matr_vec1,&matr_vec2);
    }
    else if instruct_string == "sub"{
        result_vec = sub_matr(&matr_vec1,&matr_vec2);
    }

    return create_formatted_string_from_vec(result_vec);
}