use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

pub fn scalar_multiplication(matr1: &Vec<Vec<f64>>, matr2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {

    let rows = matr1.len();
    let cols = matr2[0].len();

    let mut result_vec: Vec<Vec<f64>> = vec![vec![0.0; cols]; rows];

    let oneRows = matr1.len();
    let twoCols = matr2[0].len();
    let oneCols = matr1[0].len();
    
    for i in 0..oneRows{
        for j in 0..twoCols{
            let mut sum:f64 = 0.0;
            for k in 0..oneCols{
                sum = sum + (matr1[i][k] * matr2[k][j]);
            }
            if( i>result_vec.len()-1 ){
                break;
            }else if( j>result_vec[0].len()-1 ){
                break;
            }
            result_vec[i][j] = sum;
        }
    }
    return result_vec; 
}

pub fn mult_into_matr(multiplier: &Vec<Vec<f64>>, matr: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {

    let rows = matr.len();
    let cols = matr[0].len();


    let mut result_vec: Vec<Vec<f64>> = vec![vec![0.0; cols]; rows];

    for i in 0..matr.len(){
        for j in 0..matr[0].len(){
            result_vec[i][j] = multiplier[0][0] * matr[i][j];
        }
    }
    return result_vec;
}

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
        let col_vec: Vec<&str> = row_vec[i].split(";").collect();
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
                to_add = to_add + ";";
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

pub fn matr_transpose(matr_vec: Vec<Vec<f64>>) -> Vec<Vec<f64>>{

    let rows = matr_vec.len();
    let cols = matr_vec[0].len();

    let mut result_vec: Vec<Vec<f64>> = vec![vec![0.0; rows]; cols];

    for i in 0..cols{
        for j in 0..rows{
            if i > result_vec.len()-1 {
                break;
            }else if j > result_vec[0].len()-1 {
                break;
            }
            result_vec[i][j] = matr_vec[j][i];
        }
    }

    return result_vec;
}

//inverted matrix functons-------------------------------------------------------------------------------------
pub fn cofactor(matr_vec: &mut Vec<Vec<f64>>, p:i32, q:i32, n:i32) -> Vec<Vec<f64>>{

    let mut i: i32 = 0;
    let mut j: i32 = 0;

    let rows = matr_vec.len();
    let cols = matr_vec[0].len();

    let mut out_vec: Vec<Vec<f64>> = vec![vec![0.0; rows]; cols];

    for row in 0..n{
        for col in 0..n{
            if row != p && col != q {
                out_vec[i as usize][(j+1) as usize] = matr_vec[row as usize][col as usize];

                if j == n-1 {
                    j = 0;
                    i = i+1;
                }
            }
        }
    }
    return out_vec;
}

pub fn determinant_wrapper(matr_vec: Vec<Vec<f64>>, n:i32) -> f64 {

    let rows: usize = matr_vec.len();
    let cols: usize = matr_vec[0].len();

    let mut temp_vec: Vec<Vec<f64>> = vec![vec![0.0; rows]; cols];

    let mut rev_vec = &mut temp_vec;
    let ref_to_ref_vec = &mut rev_vec;

    let result: f64 = determinant(rev_vec, n);

    return result;
}

pub fn determinant(matr_vec: &mut Vec<Vec<f64>>, n:i32) -> f64 {
    let mut d: f64 = 0.0;

    if n == 1 {
        return matr_vec[0][0];
    }

    let mut sign: i32 = 1;

    for f in 0..n {
        let mut new_vec = cofactor(matr_vec, 0, f, n);
        let mut new_r = &mut new_vec; 
        d += sign as f64 * &matr_vec[0 as usize][f as usize] * determinant(new_r, n-1);
        sign = -sign;
    }

    return d as f64;
}
//inverted matrix functons-------------------------------------------------------------------------------------
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
    else if instruct_string == "mult"{
        result_vec = scalar_multiplication(&matr_vec1,&matr_vec2);
    }
    else if instruct_string == "mult_f64_into_matr"{
        result_vec = mult_into_matr(&matr_vec1,&matr_vec2);
    }

    return create_formatted_string_from_vec(result_vec);
}

