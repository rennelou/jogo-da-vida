use ndarray::{arr2};

const SIZE: usize = 3;

fn main() {
    let mut states = arr2(&[[0, 1, 1],
                            [1, 0, 1],
                            [0, 1, 0]]);

    println!("t0\n{}", states);
    for tick in 1..4 {
        states = automaton::tick(automaton::circular_boundary, SIZE, &states);
        println!("t{}\n{}", tick, states);
    }
}