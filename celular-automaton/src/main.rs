use ndarray::{arr2};

const SIZE: usize = 3;

fn main() {
    let mut states = arr2(&[[1, 1, 1],
                            [1, 0, 0],
                            [0, 1, 0]]);

    println!("t0\n{}", states);
    for tick in 1..6 {
        states = automaton::tick(automaton::zero_boundary, SIZE, &states);
        println!("t{}\n{}", tick, states);
    }
}