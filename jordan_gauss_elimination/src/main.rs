use std::io;

fn main() {
    let mut initial_matrix: Vec<Vec<f64>> = vec![
        vec![1.0, 1.0, 2.0, 5.0],
        vec![1.0, 2.0, 3.0, 20.0],
        vec![2.0, 4.0, 5.0, 15.0],
    ];

    //    println!();
    //    println!("🦀===Please enter the size of your square matrix===🦀");
    //
    //    let size: usize = get_size();
    //    let mut a_matrix = create_augmented_matrix(size);

    //    print_matrix(size, a_matrix);

    gaussian_elimination(initial_matrix);
}

fn get_size() -> usize {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error at read input");

    let size: usize = match input.trim().parse() {
        Ok(value) => value,

        Err(_) => {
            println!("Please only enter a valid u64");
            return 0;
        }
    };
    size
}

fn input_to_f64() -> f64 {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error at read input");

    let num: f64 = match input.trim().parse() {
        Ok(value) => value,

        Err(_) => {
            println!("Please only enter a valid f64");
            return 0.0;
        }
    };
    num
}

fn print_matrix(size: usize, matrix: Vec<Vec<f64>>) {
    for row in 0..size {
        for column in 0..=size {
            print!("{} ", matrix[row][column])
        }
        println!()
    }
}

fn create_augmented_matrix(size: usize) -> Vec<Vec<f64>> {
    println!();
    println!("{size}====Please enter the coefficients of your matrix===={size}");
    println!();

    let mut augmented_matrix = vec![vec![0.0; size + 1]; size];

    for row in 0..size {
        println!("Row: {}", row + 1);

        for column in 0..size {
            println!("Please enter your component: ({},{}) ", row + 1, column + 1);
            let component = input_to_f64();
            augmented_matrix[row][column] = component;
        }

        println!("Please enter the constant of your row: {}", row + 1);
        augmented_matrix[row][size] = input_to_f64();
    }

    augmented_matrix
}

fn gaussian_elimination(mut matrix: Vec<Vec<f64>>) {

    //first row

    let n = matrix.len();

    let mut multiple = matrix[0][0] / matrix[1][0];

    for row in 0..n - 2 {
        for column in 0..=n {
            matrix[row + 1][column] = multiple * matrix[row][column] - matrix[row + 1][column];
        }
    }

    multiple = matrix[0][0] / matrix[2][0];

    println!("{:?}", matrix);

    for row in 0..n - 2 {
        for column in 0..=n {
            matrix[row + 2][column] = multiple * matrix[row + 2][column] - matrix[row][column];
        }
    }

    println!("{:?}", matrix);

    //Second Row

    multiple = matrix[1][1] / matrix[2][1];

    for row in 1..n - 1 {
        for column in 1..=n {
            matrix[row+1][column] = multiple*matrix[row+1][column] - matrix[row][column];
        }
    }

    println!("{:?}", matrix);

    //Reverse Solving
    let mut solution = [0.0; 3];

    solution[2] = matrix[2][3] / matrix[2][2];

    solution[1] = (matrix[1][3] - solution[2]*matrix[1][2]) / matrix[1][1];

    solution[0] = (matrix[0][3] - solution[2]*matrix[0][2] - solution[1]*matrix[0][1]) / matrix[0][0];

    println!("{:?}", solution);
}
