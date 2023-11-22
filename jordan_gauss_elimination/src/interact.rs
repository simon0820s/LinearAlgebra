use std::io;

pub fn get_size() -> usize {
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

pub fn input_to_f64() -> f64 {
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

pub fn print_matrix(matrix: &Vec<Vec<f32>>) {
    for row in matrix {
        for (i, val) in row.iter().enumerate() {
            if i == row.len() - 1 {
                print!("| {:5} ", val);
                continue;
            }
            print!("{:5} ", val);
        }
        println!();
    }
    println!();
}

pub fn create_augmented_matrix(size: usize) -> Vec<Vec<f64>> {
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
