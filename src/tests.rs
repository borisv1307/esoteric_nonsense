use super::*;


/* shunting_yard tests */
#[test]
fn multiply_before_add() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(14f32, yard.calculate("2 + 4 * 3").unwrap());
    assert_eq!("2 + 4 * 3 ", yard.to_string_ast());
    assert_eq!("2 4 3 * + ", yard.to_string());
}

#[test]
fn parenthesis_overrides_multiply_before_add() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(18f32, yard.calculate("(2 + 4) * 3").unwrap());
    assert_eq!("( 2 + 4 ) * 3 ", yard.to_string_ast());
    assert_eq!("2 4 + 3 * ", yard.to_string());
}

#[test]
fn multiply_before_subtract() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(-10f32, yard.calculate("2 - 4 * 3").unwrap());
    assert_eq!("2 - 4 * 3 ", yard.to_string_ast());
    assert_eq!("2 4 3 * - ", yard.to_string());
}

#[test]
fn parenthesis_overrides_multiply_before_subtract() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(-6f32, yard.calculate("(2 - 4) * 3").unwrap());
    assert_eq!("( 2 - 4 ) * 3 ", yard.to_string_ast());
    assert_eq!("2 4 - 3 * ", yard.to_string());
}

#[test]
fn divide_before_add() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(7f32, yard.calculate("2 + 20 / 4").unwrap());
    assert_eq!("2 + 20 / 4 ", yard.to_string_ast());
    assert_eq!("2 20 4 / + ", yard.to_string());
}

#[test]
fn parenthesis_overrides_divide_before_add() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(6f32, yard.calculate("(4 + 20) / 4").unwrap());
    assert_eq!("( 4 + 20 ) / 4 ", yard.to_string_ast());
    assert_eq!("4 20 + 4 / ", yard.to_string());
}

#[test]
fn divide_before_subtract() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(-3f32, yard.calculate("2 - 20 / 4").unwrap());
    assert_eq!("2 - 20 / 4 ", yard.to_string_ast());
    assert_eq!("2 20 4 / - ", yard.to_string());
}

#[test]
fn parenthesis_overrides_divide_before_subtract() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(4f32, yard.calculate("(20 - 4) / 4").unwrap());
    assert_eq!("( 20 - 4 ) / 4 ", yard.to_string_ast());
    assert_eq!("20 4 - 4 / ", yard.to_string());
}

#[test]
fn powers_before_everything() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(55f32, yard.calculate("1 + 2 * 3 ^ 3").unwrap());
    assert_eq!("1 + 2 * 3 ^ 3 ", yard.to_string_ast());
    assert_eq!("1 2 3 3 ^ * + ", yard.to_string());
}

#[test]
fn parenthesis_overrides_powers_before_everything() {
    let mut yard = s_y::ShuntingYard::new();
    assert_eq!(217f32, yard.calculate("1 + (2 * 3) ^ 3").unwrap());
    assert_eq!("1 + ( 2 * 3 ) ^ 3 ", yard.to_string_ast());
    assert_eq!("1 2 3 * 3 ^ + ", yard.to_string());
}

/* extern tests */



/* expression tests*/
#[test]
fn add_test() {
    let u_vec: Vec<f32> = vec![5.0, 15.0];
    let v_vec: Vec<f32> = vec![6.0, 10.0];
    let results: Vec<f32> = function_maps::_add(u_vec, v_vec);

    assert_eq!(results, vec![11.0, 25.0]);
}

#[test]
fn test_subtract() {
    let u_vec: Vec<f32> = vec![10.0, 3.0];
    let v_vec: Vec<f32> = vec![4.5, 6.0];
    let results: Vec<f32> = function_maps::_subtract(u_vec, v_vec);
    assert_eq!(results, vec![5.5, -3.0]);
}

#[test]
fn test_multiply() {
    let u_vec: Vec<f32> = vec![12.0, 15.0];
    let v_vec: Vec<f32> = vec![3.0, 4.0];
    let results: Vec<f32> = function_maps::_multiply(u_vec, v_vec);
    assert_eq!(results, vec![36.0, 60.0]);
}

#[test]
fn test_n_multiply() {
    let u_vec: Vec<f32> = vec![15.0, 20.0];
    let results: Vec<f32> = function_maps::_n_multiply(u_vec, 2.0);
    assert_eq!(results, vec![30.0, 40.0]);
}

#[test]
fn test_divide() {
    let u_vec: Vec<f32> = vec![15.0, 24.0];
    let v_vec: Vec<f32> = vec![3.0, 2.0];
    let results: Vec<f32> = function_maps::_divide(u_vec, v_vec);
    assert_eq!(results, vec![5.0, 12.0]);
}

#[test]
fn test_power() {
    let u_vec: Vec<f32> = vec![3.0, 2.0, 4.0];
    let v_vec: Vec<f32> = vec![3.0, 10.0, -1.0];
    let results: Vec<f32> = function_maps::_power(u_vec, v_vec);
    assert_eq!(results, vec![27.0, 1024.0, 0.25]);
}

#[test]
fn test_n_power_u() {
    let u_vec: Vec<f32> = vec![3.0, 4.0];
    let results: Vec<f32> = function_maps::_n_power_u(u_vec, 2.0);
    assert_eq!(results, vec![8.0, 16.0]);

}

#[test]
fn test_u_power_n() {
    let u_vec: Vec<f32> = vec![3.0, 4.0];
    let results: Vec<f32> = function_maps::_u_power_n(u_vec, 2.0);
    assert_eq!(results, vec![9.0, 16.0]);

}

#[test]
fn test_sin() {
    let u_vec: Vec<f32> = vec![0.0];
    let results: Vec<f32> = function_maps::_sin(u_vec);
    assert_eq!(results, vec![0.0]);
}

#[test]
fn test_cos() {
    let u_vec: Vec<f32> = vec![0.0];
    let results: Vec<f32> = function_maps::_cos(u_vec);
    assert_eq!(results, vec![1.0]);
}

#[test]
fn test_tan() {
    let u_vec: Vec<f32> = vec![0.0];
    let results: Vec<f32> = function_maps::_tan(u_vec);
    assert_eq!(results, vec![0.0]);
}

#[test]
fn test_csc() {
    let i = std::f32::consts::PI / 2.0;
    let u_vec: Vec<f32> = vec![i];
    let results: Vec<f32> = function_maps::_csc(u_vec);
    assert_eq!(results, vec![1.0]);
}

#[test]
fn test_sec() {
    let u_vec: Vec<f32> = vec![0.0];
    let results: Vec<f32> = function_maps::_sec(u_vec);
    assert_eq!(results, vec![1.0]);
}

#[test]
fn test_cot() {
    let i = std::f32::consts::PI / 4.0;
    let u_vec: Vec<f32> = vec![i];
    let results: Vec<f32> = function_maps::_cot(u_vec);
    assert_eq!(results, vec![1.0]);
}

// TODO: tests
//  sinh(u), cosh(u), tanh(u), asin(u), acos(u), atan(u), asinh(u), acosh(u), atanh(u)

#[test]
fn test_log10() {
    let u_vec = vec![10.0, 1000.0];
    let results: Vec<f32> = function_maps::_log10(u_vec);
    assert_eq!(results, [1.0, 3.0]);
}

#[test]
fn test_ln() {
    let u_vec = vec![1.0]; 
    let results: Vec<f32> = function_maps::_ln(u_vec);
    assert_eq!(results, [0.0]);
}

#[test]
fn test_abs() {
    let u_vec = vec![-5.0];
    let results: Vec<f32> = function_maps::_abs(u_vec);
    assert_eq!(results, [5.0]);
}

#[test]
fn test_sqrt() {
    let u_vec = vec![9.0, 81.0];
    let results: Vec<f32> = function_maps::_sqrt(u_vec);
    assert_eq!(results, [3.0, 9.0]);
}
