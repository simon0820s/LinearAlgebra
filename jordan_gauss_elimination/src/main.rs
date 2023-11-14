use std::io;

fn main() {
    println!();
    println!("🦀===Please enter the size of your square matrix===🦀");
    let size: usize = input_to_u64();
    let a_matrix = create_augmented_matrix(size);
    println!("printing");
    print_matrix(size, a_matrix)
}

fn input_to_u64() -> usize {
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

fn print_matrix(size: usize, matrix: Vec<Vec<usize>>) {
    for row in 0..size {
        for column in 1..size+1{
            println!("{}", matrix[row][column])
        }
    }
}

fn create_augmented_matrix(size: usize) -> Vec<Vec<usize>> {
    println!();
    println!("{size}====Please enter the coefficients of your matrix===={size}");
    println!();

    let mut augmented_matrix = vec![vec![0; size+1]; size];

    for row in 0..size {

        println!("Row: {}", row+1);

        for column in 0..size {
            println!("Please enter your component: ({},{}) ", row+1, column+1);
            println!("({},{})", row, column);
            let component = input_to_u64();
            augmented_matrix[row][column] = component;
        }

        println!("Please enter the constant of your row: {}", row+1);
        augmented_matrix[row][size] = input_to_u64();
    }
    
    augmented_matrix
}
