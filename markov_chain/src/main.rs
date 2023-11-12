const INITIAL_STATE: [u16; 3] = [2000, 40000, 18000];
const TRANSITION_MATRIZ: [[f32; 3]; 3] = [
    [0.8, 0.2, 0.05],
    [0.05, 0.75, 0.05],
    [0.15, 0.05, 0.9]];
fn main() {
    print_initial_state();
    
    let state: [f32; 3] = state_by_time(25);

    print_state(state)
}

fn print_state(state: [f32; 3]) {

    for (index, item) in state.iter().enumerate() {
        println!("{}: {}", (index + 1), item);
    }
}

fn print_initial_state() {
    println!("====initial state====");
    println!("");

    for (index, item) in INITIAL_STATE.iter().enumerate() {
        println!("{}: {}", (index + 1), item);
    }

    println!();

    println!("===Transition Matriz===");

    for m_item in TRANSITION_MATRIZ {
        for item in m_item {
            print!(" {}", item)
        }
        println!() 
    }
}

fn state_by_time (time: u8) -> [f32; 3] {
    // x_k+1 = Px_k

    let mut state = [0.0; 3];

    let mut i: f32 = 0.0;

    for (m_index, m_item) in TRANSITION_MATRIZ.iter().enumerate() {
        for (index, item) in m_item.iter().enumerate() {
            i += item * f32::from(INITIAL_STATE[index])
        }
        state[m_index] += i;
        i = 0.0
    }

    return state;
}