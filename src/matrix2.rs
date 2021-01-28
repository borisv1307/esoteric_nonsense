/* how to use ref to ref
fn main() {
    println!("Hello, world!");
    let input = "1.0;2.0;3.0;4.0;5.0!5.0;4.0;3.0;2.0;1.0!0.0;0.0;6.0;5.0;1.0!0.0;0.0;0.0;7.0;4.0!0.0;0.0;0.0;2.0;3.0";

    let mut matrix = vec_vec_from_str(input);
    
    let mut m1: Vec<Vec<f64>> = vec![vec![1.0,2.0],vec![3.0,4.0]];
    let mut r_m1 = &mut m1;
    let rr_m1 = &mut r_m1;
    let mut m2: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0, 4.0], vec![4.0, 5.0, 6.0, 7.0], vec![7.0, 8.0, 9.0, 10.0], vec![10.0, 11.0, 12.0, 13.0]];
    let mut r_m2 = &mut m2;
    let rr_m2 = &mut r_m2;
    let mut m3: Vec<Vec<f64>> = vec![vec![0.0, 1.0, 2.0, 3.0, 4.0],
                                vec![5.0, 6.0, 7.0, 8.0, 9.0],
                                vec![10.0, 11.0, 12.0, 13.0, 14.0],
                                vec![15.0, 16.0, 17.0, 18.0, 19.0], 
                                vec![20.0, 21.0, 22.0, 23.0, 24.0]];
    let mut r_m3 = &mut m3;
    let rr_m3 = &mut r_m3;

    println!("Determinant of m1: {}", determinant(rr_m1));
    println!("Permanent of m1: {}", permanent(rr_m1));

    println!("Determinant of m2: {}", determinant(rr_m2));
    println!("Permanent of m2: {}", permanent(rr_m2));

    println!("Determinant of m3: {}", determinant(rr_m3));
    println!("Permanent of m3: {}", permanent(rr_m3));

    //println!("{:?}", matrix);
    //let str_mat = string_from_vec_vec(matrix);
    //println!("{}",str_mat);

    let mut r_mat = &mut matrix;
    let rr_mat = &mut r_mat;
    println!("matrix: {:?}", rr_mat);
    let det = determinant(rr_mat);
    println!("Determinant: {}", det);
    let perm = permanent(rr_mat);
    println!("Permanent: {}", perm);
    let scaled = scalar_multiplication(rr_mat, 3.0);
    println!("Scaled by 3: {:?}", scaled);

    let transpose = matrix_transpose(rr_mat);
    println!("Transpose:\n{:?}", transpose);

}
*/

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

//Begin Determinant/Permanent

pub fn minor( a: &mut Vec<Vec<f64>>, x: usize, y: usize) ->  Vec<Vec<f64>> {
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

pub fn determinant (matrix: &mut Vec<Vec<f64>>) -> f64 {
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

pub fn permanent (matrix: &mut Vec<Vec<f64>>) -> f64 {
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

//Begin Scalar Multiplication

pub fn scalar_multiplication(matrix: &mut Vec<Vec<f64>>, scalar: f64 ) -> Vec<Vec<f64>> {
    matrix.iter().map(|x| x.iter().map(|y| y * scalar).collect()).collect()
}

//End Scalar Multiplication

//Begin I/O

pub fn vec_vec_from_str(input: &str) -> Vec<Vec<f64>> {
    //TODO: Last ! causes problems
    let matrix: Vec<Vec<f64>> = input.replace('$', "").split("@").collect::<Vec<&str>>().iter().map(|x| x.split(";").collect::<Vec<&str>>().iter().map(|x| x.parse::<f64>().unwrap()).collect::<Vec<f64>>()).collect::<Vec<Vec<f64>>>();
    matrix
}

pub fn string_from_vec_vec (matrix: Vec<Vec<f64>>) -> String {
    //TODO: No final !
    let matrix: Vec<Vec<String>> = matrix.iter().map(|y| y.iter().map(ToString::to_string).collect()).collect();
    format!("{:?}", matrix).replace('"', "").replace("]]", "").replace("],", "@").replace("[","").replace(",", ";").replace(" ", "")   
}

//End I/O