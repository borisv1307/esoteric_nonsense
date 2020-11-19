use super::*;


/* shunting_yard tests */
#[test]
fn multiply_before_add() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(14f64, yard.calculate("2 + 4 * 3").unwrap());
    assert_eq!("2 + 4 * 3 ", yard.to_string_ast());
    assert_eq!("2 4 3 * + ", yard.to_string());
}

#[test]
fn parenthesis_overrides_multiply_before_add() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(18f64, yard.calculate("(2 + 4) * 3").unwrap());
    assert_eq!("( 2 + 4 ) * 3 ", yard.to_string_ast());
    assert_eq!("2 4 + 3 * ", yard.to_string());
}

#[test]
fn multiply_before_subtract() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(-10f64, yard.calculate("2 - 4 * 3").unwrap());
    assert_eq!("2 - 4 * 3 ", yard.to_string_ast());
    assert_eq!("2 4 3 * - ", yard.to_string());
}

#[test]
fn parenthesis_overrides_multiply_before_subtract() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(-6f64, yard.calculate("(2 - 4) * 3").unwrap());
    assert_eq!("( 2 - 4 ) * 3 ", yard.to_string_ast());
    assert_eq!("2 4 - 3 * ", yard.to_string());
}

#[test]
fn divide_before_add() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(7f64, yard.calculate("2 + 20 / 4").unwrap());
    assert_eq!("2 + 20 / 4 ", yard.to_string_ast());
    assert_eq!("2 20 4 / + ", yard.to_string());
}

#[test]
fn parenthesis_overrides_divide_before_add() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(6f64, yard.calculate("(4 + 20) / 4").unwrap());
    assert_eq!("( 4 + 20 ) / 4 ", yard.to_string_ast());
    assert_eq!("4 20 + 4 / ", yard.to_string());
}

#[test]
fn divide_before_subtract() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(-3f64, yard.calculate("2 - 20 / 4").unwrap());
    assert_eq!("2 - 20 / 4 ", yard.to_string_ast());
    assert_eq!("2 20 4 / - ", yard.to_string());
}

#[test]
fn parenthesis_overrides_divide_before_subtract() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(4f64, yard.calculate("(20 - 4) / 4").unwrap());
    assert_eq!("( 20 - 4 ) / 4 ", yard.to_string_ast());
    assert_eq!("20 4 - 4 / ", yard.to_string());
}

#[test]
fn powers_before_everything() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(55f64, yard.calculate("1 + 2 * 3 ^ 3").unwrap());
    assert_eq!("1 + 2 * 3 ^ 3 ", yard.to_string_ast());
    assert_eq!("1 2 3 3 ^ * + ", yard.to_string());
}

#[test]
fn parenthesis_overrides_powers_before_everything() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(217f64, yard.calculate("1 + (2 * 3) ^ 3").unwrap());
    assert_eq!("1 + ( 2 * 3 ) ^ 3 ", yard.to_string_ast());
    assert_eq!("1 2 3 * 3 ^ + ", yard.to_string());
}

/* extern tests */



/* expression tests*/
#[test]
fn test_add() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(23.0, yard.calculate("11 + 12").unwrap());
}

#[test]
fn test_subtract() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(4.0, yard.calculate("7 - 3").unwrap());
}

#[test]
fn test_multiply() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(15.0, yard.calculate("3 * 5").unwrap());
}

#[test]
fn test_divide() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(10.0, yard.calculate("20 / 2").unwrap());
}

#[test]
fn test_power() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(27.0, yard.calculate("3^3").unwrap());
}

#[test]
fn test_sin() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(0.0, yard.calculate("sin(0)").unwrap());

}

#[test]
fn test_cos() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(1.0, yard.calculate("cos(0)").unwrap());

}

#[test]
fn test_tan() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(0.0, yard.calculate("tan(0)").unwrap());
}

#[test]
fn test_sec() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(1.0, yard.calculate("sec(0)").unwrap());
}

// TODO: tests
//  csc, cot, sinh, cosh, tanh, asin, acos, atan, asinh, acosh, atanh

#[test]
fn test_log() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(3.0, yard.calculate("log(1000)").unwrap());
}

#[test]
fn test_ln() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(0.0, yard.calculate("ln(1)").unwrap());
}

#[test]
fn test_abs() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(30.0, yard.calculate("abs(-30)").unwrap());
}

#[test]
fn test_sqrt() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(5.0, yard.calculate("sqrt(25)").unwrap());
}

//matrix tests
#[test]
fn test_add_matr() {

    let test_matr1: Vec<Vec<f64>> = vec![vec![1.0,2.0,3.0],vec![1.0,2.0,3.0],vec![1.0,2.0,3.0]];
    let test_matr2: Vec<Vec<f64>> = vec![vec![1.0,2.0,3.0],vec![1.0,2.0,3.0],vec![1.0,2.0,3.0]];

    let test_result: Vec<Vec<f64>> = matrices::add_matr(&test_matr1,&test_matr2);
        assert_eq!(test_result, vec![vec![2.0,4.0,6.0],vec![2.0,4.0,6.0],vec![2.0,4.0,6.0]]);
}

#[test]
fn test_sub_matr() {

    let test_matr1: Vec<Vec<f64>> = vec![vec![1.0,2.0,3.0],vec![1.0,2.0,3.0],vec![1.0,2.0,3.0]];
    let test_matr2: Vec<Vec<f64>> = vec![vec![1.0,2.0,3.0],vec![1.0,2.0,3.0],vec![1.0,2.0,3.0]];

    let test_result: Vec<Vec<f64>> = matrices::sub_matr(&test_matr1,&test_matr2);
        assert_eq!(test_result, vec![vec![0.0,0.0,0.0],vec![0.0,0.0,0.0],vec![0.0,0.0,0.0]]);

}

#[test]
fn test_string_to_vec_matr() {

    let test = String::from("1,2,3!1,2,3!1,2,3");
    let test2 = CString::new(test).unwrap();
    let mut test3: Vec<u8> = test2.into_bytes_with_nul();
    let _ctest: *mut i8 = test3.as_mut_ptr() as *mut i8;
    let test_vec: Vec<Vec<f64>> = matrices::create_vec_from_string(_ctest);
        assert_eq!(test_vec, vec![vec![1.0,2.0,3.0],vec![1.0,2.0,3.0],vec![1.0,2.0,3.0]]);
}

#[test]
fn test_matr_vec_to_string() {

    //create test c_char
    let test_string = String::from("1,2,3!1,2,3!1,2,3");

    let formatted_string: *mut c_char = matrices::create_formatted_string_from_vec(vec![vec![1.0,2.0,3.0],vec![1.0,2.0,3.0],vec![1.0,2.0,3.0]]);
    let formatted_string: &CStr = unsafe { CStr::from_ptr(formatted_string)};
    let test: String = formatted_string.to_str().unwrap().to_string();

    assert_eq!(test_string, test);

}

#[test]
fn test_matr_calc_add() {

    let test_alpha = String::from("1,2,3!1,2,3!1,2,3");
    let test2_alpha = CString::new(test_alpha).unwrap();
    let mut test3_alpha: Vec<u8> = test2_alpha.into_bytes_with_nul();
    let ctest_alpha: *mut i8 = test3_alpha.as_mut_ptr() as *mut i8;

    let test_beta = String::from("1,2,3!1,2,3!1,2,3");
    let test2_beta = CString::new(test_beta).unwrap();
    let mut test3_beta: Vec<u8> = test2_beta.into_bytes_with_nul();
    let ctest_beta: *mut i8 = test3_beta.as_mut_ptr() as *mut i8;

    let test_instr = String::from("add");
    let test2_instr = CString::new(test_instr).unwrap();
    let mut test3_instr: Vec<u8> = test2_instr.into_bytes_with_nul();
    let ctest_instr: *mut i8 = test3_instr.as_mut_ptr() as *mut i8;

    let result_test_raw: *mut c_char = matrices::calculate_matr(ctest_alpha,ctest_beta,ctest_instr);

    let back_to_string1: &CStr = unsafe { CStr::from_ptr(result_test_raw)};
    let back_to_string2: String = back_to_string1.to_str().unwrap().to_string();

    assert_eq!("2,4,6!2,4,6!2,4,6", back_to_string2);

}

#[test]
fn test_matr_calc_sub() {

    let test_alpha = String::from("1,2,3!1,2,3!1,2,3");
    let test2_alpha = CString::new(test_alpha).unwrap();
    let mut test3_alpha: Vec<u8> = test2_alpha.into_bytes_with_nul();
    let ctest_alpha: *mut i8 = test3_alpha.as_mut_ptr() as *mut i8;

    let test_beta = String::from("1,2,3!1,2,3!1,2,3");
    let test2_beta = CString::new(test_beta).unwrap();
    let mut test3_beta: Vec<u8> = test2_beta.into_bytes_with_nul();
    let ctest_beta: *mut i8 = test3_beta.as_mut_ptr() as *mut i8;

    let test_instr = String::from("sub");
    let test2_instr = CString::new(test_instr).unwrap();
    let mut test3_instr: Vec<u8> = test2_instr.into_bytes_with_nul();
    let ctest_instr: *mut i8 = test3_instr.as_mut_ptr() as *mut i8;

    let result_test_raw: *mut c_char = matrices::calculate_matr(ctest_alpha,ctest_beta,ctest_instr);

    let back_to_string1: &CStr = unsafe { CStr::from_ptr(result_test_raw)};
    let back_to_string2: String = back_to_string1.to_str().unwrap().to_string();

    assert_eq!("0,0,0!0,0,0!0,0,0", back_to_string2);

}