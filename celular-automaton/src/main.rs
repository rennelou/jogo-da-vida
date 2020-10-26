use ndarray::{arr2};

fn main() {
    let mut states = arr2(&[[0, 1, 1],
                            [1, 0, 1],
                            [0, 1, 0]]);

    println!("t0\n{}", states);
    for tick in 1..4 {
        states = automaton::tick(&states);
        println!("t{}\n{}", tick, states);
    }
}