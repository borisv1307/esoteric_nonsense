pub fn commander(command: &str, matrices_in: &str, scalar: f64) -> String {
    let mut matrices_vec: Vec<Vec<Vec<f64>>> = parse_matrices(matrices_in);
    let mut r_mat_vec = &mut matrices_vec;
    let rr_mat = &mut r_mat_vec;
    let mut result: String = "".to_string();
    match command {
        "add" => {
            let matrix_a: Vec<Vec<f64>> = rr_mat[0].iter().map(|a| a.to_owned()).collect::<Vec<Vec<f64>>>();
            let matrix_b: Vec<Vec<f64>> = rr_mat[1].iter().map(|a| a.to_owned()).collect::<Vec<Vec<f64>>>(); 
            result = string_from_vec_vec(add_matrices(matrix_a, matrix_b));
        },
        "subtract" => {
            let matrix_a: Vec<Vec<f64>> = rr_mat[0].iter().map(|a| a.to_owned()).collect::<Vec<Vec<f64>>>();
            let matrix_b: Vec<Vec<f64>> = rr_mat[1].iter().map(|a| a.to_owned()).collect::<Vec<Vec<f64>>>(); 
            result = string_from_vec_vec(subtract_matrices(matrix_a, matrix_b));
        },
        "scalar_multiplication" => {
            result = string_from_vec_vec(scalar_multiplication(&mut rr_mat[0], scalar));
        },
        "transpose" => {
            result = string_from_vec_vec(matrix_transpose(&mut rr_mat[0]));
        },
        "determinant" => {
            result = determinant(&mut rr_mat[0]).to_string();
        },
        "permanent" => {
            result = permanent(&mut rr_mat[0]).to_string();
        },

        _ => {
            println!("invalid command");
        }
    }

    result
}

//Begin Matrix Subtraction

fn subtract_matrices(matrix_a: Vec<Vec<f64>>, matrix_b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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

fn add_matrices(matrix_a: Vec<Vec<f64>>, matrix_b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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

fn scalar_multiplication(matrix: &mut Vec<Vec<f64>>, scalar: f64 ) -> Vec<Vec<f64>> {
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

fn determinant(matrix: &mut Vec<Vec<f64>>) -> f64 {
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

fn permanent(matrix: &mut Vec<Vec<f64>>) -> f64 {
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

fn matrix_transpose(matrix: &mut Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut transpose = vec![Vec::with_capacity(matrix.len()); matrix[0].len()];
    for row in matrix {
        for i in 0..row.len() {
            transpose[i].push(row[i]);
        }
    }
    transpose
}

//End Transpose

//Begin I/O

fn parse_matrices(matrices_in: &str) -> Vec<Vec<Vec<f64>>> {
    let matrices_parsed: Vec<Vec<Vec<f64>>> = matrices_in.split("$&").collect::<Vec<&str>>().iter().map(|i| i.split("@").collect::<Vec<&str>>().iter().map(|x| x.split(";").collect::<Vec<&str>>().iter().map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>()).collect::<Vec<Vec<f64>>>()).collect::<Vec<Vec<Vec<f64>>>>();
    matrices_parsed
}

fn string_from_vec_vec(matrix: Vec<Vec<f64>>) -> String {
    //TODO: does Alisha need this formatted differently?
    let matrix: Vec<Vec<String>> = matrix.iter().map(|y| y.iter().map(ToString::to_string).collect()).collect();
    format!("{:?}", matrix).replace('"', "").replace("]]", "").replace("],", "@").replace("[","").replace(",", ";").replace(" ", "")   
}

//End I/O