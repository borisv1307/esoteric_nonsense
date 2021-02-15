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

#[test]
fn test_determinant_a(){
    let mut m1: Vec<Vec<f64>> = vec![vec![1.0,2.0],vec![3.0,4.0]];
    let mut r_m1 = &mut m1;
    let rr_m1 = &mut r_m1;
    assert_eq!(matrix::determinant(rr_m1), -2.0);
}

#[test]
fn test_det_b(){
    let mut m2: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0, 4.0], vec![4.0, 5.0, 6.0, 7.0], vec![7.0, 8.0, 9.0, 10.0], vec![10.0, 11.0, 12.0, 13.0]];
    let mut r_m2 = &mut m2;
    let rr_m2 = &mut r_m2;
    assert_eq!(matrix::determinant(rr_m2), 0.0);
}

#[test]
fn test_det_c(){
    let mut m3: Vec<Vec<f64>> = vec![vec![0.0, 1.0, 2.0, 3.0, 4.0],
                                vec![5.0, 6.0, 7.0, 8.0, 9.0],
                                vec![10.0, 11.0, 12.0, 13.0, 14.0],
                                vec![15.0, 16.0, 17.0, 18.0, 19.0], 
                                vec![20.0, 21.0, 22.0, 23.0, 24.0]];
    let mut r_m3 = &mut m3;
    let rr_m3 = &mut r_m3;
    assert_eq!(matrix::determinant(rr_m3), 0.0);
}

#[test]
fn test_permanent_a(){
    let mut m1: Vec<Vec<f64>> = vec![vec![1.0,2.0],vec![3.0,4.0]];
    let mut r_m1 = &mut m1;
    let rr_m1 = &mut r_m1;
    assert_eq!(matrix::permanent(rr_m1), 10.0);
}

#[test]
fn test_perm_b(){
    let mut m2: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0, 4.0], vec![4.0, 5.0, 6.0, 7.0], vec![7.0, 8.0, 9.0, 10.0], vec![10.0, 11.0, 12.0, 13.0]];
    let mut r_m2 = &mut m2;
    let rr_m2 = &mut r_m2;
    assert_eq!(matrix::permanent(rr_m2), 29556.0);
}

#[test]
fn test_perm_c(){
    let mut m3: Vec<Vec<f64>> = vec![vec![0.0, 1.0, 2.0, 3.0, 4.0],
                                vec![5.0, 6.0, 7.0, 8.0, 9.0],
                                vec![10.0, 11.0, 12.0, 13.0, 14.0],
                                vec![15.0, 16.0, 17.0, 18.0, 19.0], 
                                vec![20.0, 21.0, 22.0, 23.0, 24.0]];
    let mut r_m3 = &mut m3;
    let rr_m3 = &mut r_m3;
    assert_eq!(matrix::permanent(rr_m3), 6778800.0);
}

#[test]
fn test_transpose(){
    let mut m = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let n = vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]];
    assert_eq!(matrix::matrix_transpose(&mut m),n);
}

#[test]
fn test_scalar_mult(){
    let mut m = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let n = vec![vec![2.0, 4.0, 6.0], vec![8.0, 10.0, 12.0]];
    assert_eq!(matrix::scalar_multiplication(&mut m, 2.0), n);
}

#[test]
fn test_reduced_row_echelon(){
    let mut matrix_to_reduce: Vec<Vec<f64>> = vec![vec![1.0, 2.0 , -1.0, -4.0], 
                                                vec![2.0, 3.0, -1.0, -11.0],
                                                vec![-2.0, 0.0, -3.0, 22.0]];
    let mut r_mat_to_red = &mut matrix_to_reduce;
    let rr_mat_to_red = &mut r_mat_to_red;

    let reduced = vec![vec![1.0, 0.0, 0.0, -8.0], vec![-0.0, 1.0, 0.0, 1.0], vec![-0.0, -0.0, 1.0, -2.0]];
    assert_eq!(matrix::reduced_row_echelon_form(rr_mat_to_red), reduced);
}

#[test]
fn test_matrix_add(){
    let a: Vec<Vec<f64>> = vec![vec![1.0, 2.0 , -1.0, -4.0], 
                            vec![2.0, 3.0, -1.0, -11.0],
                            vec![-2.0, 0.0, -3.0, 22.0]];
    let b = vec![vec![1.0, 0.0, 0.0, -8.0],
                vec![0.0, 1.0, 0.0, 1.0], 
                vec![0.0, 0.0, 1.0, -2.0]];

    let c = vec![vec![2.0, 2.0, -1.0, -12.0], 
                vec![2.0, 4.0, -1.0, -10.0], 
                vec![-2.0, -0.0, -2.0, 20.0]];
    
    assert_eq!(matrix::add_matrices(a, b), c);

}

#[test]
fn test_matrix_subtract(){
    let a: Vec<Vec<f64>> = vec![vec![1.0, 2.0 , -1.0, -4.0], 
                            vec![2.0, 3.0, -1.0, -11.0],
                            vec![-2.0, 0.0, -3.0, 22.0]];
    let b = vec![vec![1.0, 0.0, 0.0, -8.0],
                vec![0.0, 1.0, 0.0, 1.0], 
                vec![0.0, 0.0, 1.0, -2.0]];

    let c = vec![vec![0.0, 2.0, -1.0, 4.0], 
                vec![2.0, 2.0, -1.0, -12.0], 
                vec![-2.0, -0.0, -4.0, 24.0]];
    
    assert_eq!(matrix::subtract_matrices(a, b), c);
}

#[test]
fn test_matrix_from_string() {
    let matrix_in_str = "&1.1;2.0@3.0;4.0@5.0;6.0$";
    let matrix = vec![vec![1.1, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
    assert_eq!(matrix::parse_matrices(matrix_in_str), matrix);
}

#[test]
fn test_string_from_matrix() {
    let matrix = vec![vec![1.1, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
    let matrix_str = "&1.1;2@3;4@5;6$".to_string();
    assert_eq!(matrix::string_from_vec_vec(matrix), matrix_str);
}

#[test]
fn test_matrix_multiplication() {
    let mut m3: Vec<Vec<f64>> = vec![vec![0.0, 1.0, 2.0, 3.0, 4.0],
                                vec![5.0, 6.0, 7.0, 8.0, 9.0],
                                vec![10.0, 11.0, 12.0, 13.0, 14.0],
                                vec![15.0, 16.0, 17.0, 18.0, 19.0], 
                                vec![20.0, 21.0, 22.0, 23.0, 24.0]];
    let mut r_m3 = &mut m3;
    let rr_m3 = &mut r_m3;
    let m4: Vec<Vec<f64>> = rr_m3.iter().map(|i| i.iter().map(|j| j.to_owned()).collect()).collect();
    let m5: Vec<Vec<f64>> = matrix::matrix_transpose(rr_m3);
    let m2 = vec![vec![30.0, 80.0, 130.0, 180.0, 230.0], 
            vec![80.0, 255.0, 430.0, 605.0, 780.0], vec!
            [130.0, 430.0, 730.0, 1030.0, 1330.0], 
            vec![180.0, 605.0, 1030.0, 1455.0, 1880.0], vec!
            [230.0, 780.0, 1330.0, 1880.0, 2430.0]];
    assert_eq!(matrix::matrix_multiplication(&m4, &m5), m2);
}

#[test]
fn test_is_square_true(){
    let square_mat_true = vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]];
    assert_eq!(matrix::is_square(&square_mat_true), true);
}

#[test]
fn test_is_square_false(){
    let square_mat_false = vec![vec![1., 2., 3.], vec![4., 5., 6.]];
    assert_eq!(matrix::is_square(&square_mat_false), false);
}

#[test]
fn test_is_rref_compat_true(){
    let rref_true = vec![vec![1., 2., 3.], vec![4., 5., 6.]];
    assert_eq!(matrix::is_rref_compat(&rref_true), true);
}

#[test]
fn test_is_rref_compat_false(){
    let rref_false = vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]];
    assert_eq!(matrix::is_rref_compat(&rref_false), false);
}

#[test]
fn test_zero_matrix_a(){
    let one_by_one = vec![vec![0.0]];
    assert_eq!(matrix::zero_matrix(1, 1), one_by_one);
}

#[test]
fn test_zero_matrix_b(){
    let two_by_three = vec![vec![0.,0., 0.], vec![0.,0., 0.]];
    assert_eq!(matrix::zero_matrix(2, 3), two_by_three);
}

#[test]
fn test_zero_matrix_c(){
    let four_by_five = vec![vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0]];
    assert_eq!(matrix::zero_matrix(4, 5), four_by_five);
}

#[test]
fn test_ident_matrix_1(){
    let one = vec![vec![1.0]];
    assert_eq!(matrix::identity_matrix(1), one);
}

#[test]
fn test_ident_matrix_3(){
    let three = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]];
    assert_eq!(matrix::identity_matrix(3), three);
}

#[test]
fn test_pivot_matrix(){
    let matrix = vec![vec![11.0,9.0,24.0,2.0], 
                    vec![1.0, 5.0, 2.0, 6.0], 
                    vec![3.0, 17.0, 18.0, 1.0], 
                    vec![2.0, 5.0, 2.0, 1.0]];
    let pivot = vec![vec![1.0, 0.0, 0.0, 0.0],
                    vec![0.0, 0.0, 1.0, 0.0],
                    vec![0.0, 1.0, 0.0, 0.0],
                    vec![0.0, 0.0, 0.0, 1.0]];
    assert_eq!(matrix::pivotize_matrix(&matrix), pivot);
}

#[test]
fn test_lu_decom_prime(){
    let matrix = vec![vec![1.,3.,5.], vec![2.,4.,7.], vec![1.,1.,0.]];
    let prime = vec![vec![2.0, 4.0, 7.0], vec![1.0, 3.0, 5.0], vec![1.0, 1.0, 0.0]];
    assert_eq!(matrix::lower_upper_decomposition(&matrix).0, prime);
}

#[test]
fn test_lu_decomp_lower(){
    let matrix = vec![vec![1.,3.,5.], vec![2.,4.,7.], vec![1.,1.,0.]];
    let l = vec![vec![1.,0.,0.], vec![0.5, 1., 0.], vec![0.5,-1.,1.]];
    assert_eq!(l, matrix::lower_upper_decomposition(&matrix).1);
}

#[test]
fn test_lu_decomp_upper(){
    let matrix = vec![vec![1.,3.,5.], vec![2.,4.,7.], vec![1.,1.,0.]];
    let u = vec![vec![2.,4.,7.], vec![0.,1.,1.5], vec![0.,0.,-2.]];
    assert_eq!(matrix::lower_upper_decomposition(&matrix).2, u);
}

#[test]
fn test_lu_decomp_permute(){
    let matrix = vec![vec![1.,3.,5.], vec![2.,4.,7.], vec![1.,1.,0.]];
    let permute = vec![vec![0.,1.,0.], vec![1.,0.,0.], vec![0.,0.,1.]];
    assert_eq!(matrix::lower_upper_decomposition(&matrix).3, permute);
}

