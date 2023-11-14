use std::io;
fn main() {
    println!();
    println!("🦀===Please enter the size of your square matrix===🦀");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error at read input");

    let size: u64 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Please only enter a valid u64");
            return ;
        }
    };

    create_augmented_matrix(size);

}

fn create_augmented_matrix(size: u64) {
    println!();
    println!("🦀====Please enter the coefficients of your matrix====🦀");
    println!();
    println!("First row:");
    println!("Size: {}", size)
}
