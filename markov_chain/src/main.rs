use std::io;
const INITIAL_STATE: [f32; 3] = [2000.0, 40000.0, 18000.0];
const TRANSITION_MATRIX: [[f32; 3]; 3] = [[0.8, 0.2, 0.05], [0.05, 0.75, 0.05], [0.15, 0.05, 0.9]];

fn main() {
    print_initial_state();

    let mut n_iterations_s = String::new();

    println!("Please enter the desired number of iterations 🦀");

    io::stdin()
        .read_line(&mut n_iterations_s)
        .expect("Error at read input");

    let n_iterations: u64 = match n_iterations_s.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Please only aneter a valid u64 response");
            return;
        }
    };

    let mut previous_state: [f32; 3] = INITIAL_STATE;

    for iteration in 0..=n_iterations {
        println!("iteration: {}", iteration);
        previous_state = state_by_time(previous_state);
    }
}

fn print_state(state: [f32; 3]) {
    for (index, item) in state.iter().enumerate() {
        println!("{}: {}", (index + 1), item);
    }
}

fn print_initial_state() {
    println!("🦀====initial state====🦀");
    println!("");

    for (index, item) in INITIAL_STATE.iter().enumerate() {
        println!("{}: {}", (index + 1), item);
    }

    println!();

    println!("🦀===Transition Matriz===🦀");

    for m_item in TRANSITION_MATRIX {
        for item in m_item {
            print!(" {}", item)
        }
        println!()
    }
    println!();
}

fn state_by_time(previous_state: [f32; 3]) -> [f32; 3] {
    // x_k+1 = Px_k

    let mut state = [0.0; 3];

    let mut i: f32 = 0.0;

    for (m_index, m_item) in TRANSITION_MATRIX.iter().enumerate() {
        for (index, item) in m_item.iter().enumerate() {
            i += item * f32::from(previous_state[index])
        }
        state[m_index] += i;
        i = 0.0
    }

    print_state(state);
    println!();
    println!();
    state
}
