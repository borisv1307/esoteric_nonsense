pub fn commander(command: &str, matrix_a_in: &str, matrix_b_in: &str, scalar_1: f64, scalar_2: f64) -> String {
    let mut matrix_a: Vec<Vec<f64>> = scalar_multiplication(&mut parse_matrices(matrix_a_in), scalar_1);
    let mut r_mat_a = &mut matrix_a;
    let rr_mat_a = &mut r_mat_a;

    let mut matrix_b: Vec<Vec<f64>> = scalar_multiplication(&mut parse_matrices(matrix_b_in), scalar_2);
    let mut r_mat_b = &mut matrix_b;
    let rr_mat_b = &mut r_mat_b;
    
    let mut result: String = "".to_string();
    match command {
        "add" => {
            result = string_from_vec_vec(add_matrices(matrix_a, matrix_b));
        },
        "subtract" => {
            result = string_from_vec_vec(subtract_matrices(matrix_a, matrix_b));
        },
        "transpose" => {
            result = string_from_vec_vec(matrix_transpose(rr_mat_a));
        },
        "determinant" => {
            result = determinant(rr_mat_a).to_string();
        },
        "permanent" => {
            result = permanent(rr_mat_a).to_string();
        },
        "reduced_row_echelon" => {
            result = string_from_vec_vec(reduced_row_echelon_form(rr_mat_a))
        },
        "multiply" => {
            result = string_from_vec_vec(matrix_multiplication(r_mat_a, r_mat_b))
        }
        _ => {
            result = "invalid command".to_string();
            println!("invalid command");
        }
    }

    result
}

//Begin LU Decomposition
fn lower_upper_decomposition(matrix: &Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>,) {
    if !is_square(&matrix) {
        (vec![vec![std::f64::NAN]], vec![vec![std::f64::NAN]], vec![vec![std::f64::NAN]], vec![vec![std::f64::NAN]])
    } else {
        let n = matrix.len();
        let mut lower: Vec<Vec<f64>> = identity_matrix(n);
        let mut upper: Vec<Vec<f64>> = zero_matrix(n, n);
        let permuted = pivotize_matrix(&matrix);
        let matrix_prime = matrix_multiplication(&permuted, &matrix);

        for j in 0..n {
            lower[j][j] = 1.0;
            for i in 0..j+1 {
                let mut sum = 0.0;
                for k in 0..i {
                    sum += upper[k][j] * lower[i][k];
                }
                upper[i][j] = matrix_prime[i][j] - sum
            }
            for i in j..n {
                let mut sum = 0.0;
                for k in 0..j {
                    sum += upper[k][j] * lower[i][k];
                }
                lower[i][j] = (matrix_prime[i][j] - sum) / upper[j][j];
            }
        }
        (matrix_prime, lower, upper, permuted)
    }
    
}

fn pivotize_matrix(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut id = identity_matrix(matrix.len());
    for i in 0..matrix.len() {
        let mut max = matrix[i][i];
        let mut row = i;
        for j in i..matrix.len()-1 {
            if matrix[j][i] > max {
                max = matrix[j][i];
                row = j;
            }
        }
        if row != i {
            id.swap(row, i);
        }
    }
    id
}

fn zero_matrix(rows: usize, cols: usize) -> Vec<Vec<f64>> {
    let mut matrix = Vec::with_capacity(cols);
    for _ in 0..rows {
        let mut col: Vec<f64> = Vec::with_capacity(rows);
        for _ in 0..cols {
            col.push(0.0);
        }
        matrix.push(col);
    }
    matrix
}

fn identity_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut i_matrix = Vec::with_capacity(n);
    for i in 0..n {
        let mut col: Vec<f64> = Vec::with_capacity(n);
        for j in 0..n {
            if i == j {
                col.push(1.0);
            } else {
                col.push(0.0);
            }
        }
        i_matrix.push(col);
    }
    i_matrix
}
//End LU Decomposition


//Begin Matrix Multiplication
fn matrix_multiplication(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    let mut out = vec![vec![0.0; a.len()]; b[0].len()];
    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..a[0].len() {
                out[i][j] += a[i][k] * b[k][j]; 
            }
        }
    }
    out
}
//End Matrix Multiplication

//Begin Reduced Row Echelon Form
pub fn reduced_row_echelon_form(matrix: &mut Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut matrix_out: Vec<Vec<f64>> = matrix.to_vec();
    let mut pivot = 0;
    let row_count = matrix_out.len();
    let column_count = matrix_out[0].len();
    
    for r in 0..row_count {
        if column_count <= pivot {
            break;
        }
        let mut i = r;
        while matrix_out[i][pivot] == 0.0 {
            i = i + 1;
            if i == row_count {
                i = r;
                pivot = pivot + 1;
                if column_count == pivot {
                    pivot = pivot - 1;
                    break;
                }
            }
        }
        for j in 0..row_count {
            let temp = matrix_out[r][j];
            matrix_out[r][j] = matrix_out[i][j];
            matrix_out[i][j] = temp;
        }
        let divisor = matrix_out[r][pivot];
        if divisor != 0.0 {
            for j in 0..column_count {
                matrix_out[r][j] = matrix_out[r][j] / divisor;
            }
        }
        for j in 0..row_count {
            if j != r {
                let hold = matrix_out[j][pivot];
                for k in 0..column_count {
                    matrix_out[j][k] = matrix_out[j][k] - ( hold * matrix_out[r][k]);
                }
            }
        }
        pivot = pivot + 1;
    }
    matrix_out
}
//End Reduced Row Echelon Form

//Begin Matrix Subtraction
pub fn subtract_matrices(matrix_a: Vec<Vec<f64>>, matrix_b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = Vec::new();
    for r in 0..matrix_a.len() {
        let mut row: Vec<f64> = Vec::with_capacity(matrix_a.len());
        for c in 0..matrix_a[0].len() {
            row.push(matrix_a[r][c] - matrix_b[r][c]);
        }
        out.push(row);
    }
    out
}
//End Matrix Subtraction

//Begin Matrix Addition
pub fn add_matrices(matrix_a: Vec<Vec<f64>>, matrix_b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = Vec::new();
    for r in 0..matrix_a.len() {
        let mut row: Vec<f64> = Vec::with_capacity(matrix_a.len());
        for c in 0..matrix_a[0].len() {
            row.push(matrix_a[r][c] + matrix_b[r][c]);
        }
        out.push(row);
    }
    out
}
//End Matrix Addition

//Begin Scalar Multiplication
pub fn scalar_multiplication(matrix: &mut Vec<Vec<f64>>, scalar: f64 ) -> Vec<Vec<f64>> {
    matrix.iter().map(|x| x.iter().map(|y| y * scalar).collect()).collect()
}
//End Scalar Multiplication

//Begin Determinant/Permanent
fn minor( a: &mut Vec<Vec<f64>>, x: usize, y: usize) ->  Vec<Vec<f64>> {
    let mut out_vec: Vec<Vec<f64>> = vec![vec![0.0; a.len() - 1]; a.len() -1];
    for i in 0..a.len()-1 {
        for j in 0..a.len()-1 {
            match () {
                _ if (i < x && j < y) => {
                    out_vec[i][j] = a[i][j];
                },
                _ if (i >= x && j < y) => {
                    out_vec[i][j] = a[i + 1][j];
                },
                _ if (i < x && j >= y) => {
                    out_vec[i][j] = a[i][j + 1];
                },
                _ => { //i > x && j > y 
                    out_vec[i][j] = a[i + 1][j + 1];
                },
            }
        }
    }
    out_vec
}

pub fn determinant(matrix: &mut Vec<Vec<f64>>) -> f64 {
    match () {
        _ if (matrix.len() == 1) => {
            matrix[0][0]
        },
        _ => {
            let mut sign = 1.0;
            let mut sum = 0.0;
            for i in 0..matrix.len() {
                sum = sum + sign * matrix[0][i] * determinant(&mut minor(matrix, 0, i));
                sign = sign * -1.0;
            }
            sum
        }
    }
}

pub fn permanent(matrix: &mut Vec<Vec<f64>>) -> f64 {
    match () {
        _ if (matrix.len() == 1) => {
            matrix[0][0]
        },
        _ => {
            let mut sum = 0.0;
            for i in 0..matrix.len() {
                sum = sum + matrix[0][i] * permanent(&mut minor(matrix, 0, i));
            }
            sum
        }
    }
}
//End Determinant/Permanent

//Begin Transpose
pub fn matrix_transpose(matrix: &mut Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut transpose = vec![Vec::with_capacity(matrix.len()); matrix[0].len()];
    for row in matrix {
        for i in 0..row.len() {
            transpose[i].push(row[i]);
        }
    }
    transpose
}
//End Transpose

//Begin Check Matrices
fn is_square(matrix: &Vec<Vec<f64>>) -> bool {
    matrix.len() == matrix[0].len()
}

fn is_rref_compat(matrix: &Vec<Vec<f64>>) -> bool {
    matrix.len() + 1 == matrix[0].len()
    
}
//End Check Matrices

//Begin I/O
pub fn parse_matrices(matrix_in: &str) -> Vec<Vec<f64>> {
    let matrix_parsed: Vec<Vec<f64>> = matrix_in.replace('&', "").replace('$', "").split("@").collect::<Vec<&str>>().iter().map(|row| row.split(";").collect::<Vec<&str>>().iter().map(|element| element.parse::<f64>().unwrap()).collect::<Vec<f64>>()).collect::<Vec<Vec<f64>>>();
    matrix_parsed
}

pub fn string_from_vec_vec(matrix: Vec<Vec<f64>>) -> String {
    let matrix: Vec<Vec<String>> = matrix.iter().map(|y| y.iter().map(ToString::to_string).collect()).collect();
    format!("{:?}", matrix).replace('"', "").replace("[[","&").replace("]]", "$").replace("],", "@").replace("[","").replace(",", ";").replace(" ", "")   
}
//End I/O