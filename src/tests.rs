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
