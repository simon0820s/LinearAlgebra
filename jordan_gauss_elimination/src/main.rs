mod interact;
use interact::print_matrix;
fn main() {
    let matrix: Vec<Vec<f32>> = vec![
        vec![2., 4., 6., 18.],
        vec![4., 5., 6., 24.],
        vec![3., 1., -2., 4.],
    ];
    let result = jordan_gauss_elimination(matrix);
    println!("{:?}", result)
}

fn jordan_gauss_elimination(mut matrix: Vec<Vec<f32>>) -> Vec<f64> {
    let mut result = vec![0.0; 3];

    let n_r: usize = matrix.len();
    let n_c: usize = matrix[0].len();

    //staggered matrix

    for row in 0..n_r - 1 {
        for r in row..n_r - 1 {
            
            let multiple: f32 = if matrix[r + 1][row] != 0.0 {
                matrix[row][row] / matrix[r + 1][row]
            } else {
                continue;
            };
            println!("{}", multiple);

            for c in row..n_c {
                matrix[r + 1][c] = round_to( multiple * matrix[r + 1][c] - matrix[row][c], 3 )
            }
        }
        print_matrix(&matrix);
    }

    //backward substitution
    result
}

fn round_to(num: f32, decimal_places: usize) -> f32 {
    let multiplier = 10_f32.powi(decimal_places as i32);
    (num * multiplier).round() / multiplier
}
