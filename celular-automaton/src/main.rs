use ndarray::{arr2};
use automaton::{Grid, Boundary};

fn main() {
    let mut states = arr2(&[[0, 0, 0, 0, 0],
                            [0, 0, 0, 0, 0],
                            [0, 0, 1, 1, 1],
                            [0, 0, 1, 0, 0],
                            [0, 0, 0, 1, 0]]);

    let grid = Grid::new(&states, Boundary::Limited);
    println!("t0\n{}", states);
    for tick in 1..6 {
        states = grid.tick(&states);
        println!("t{}\n{}", tick, states);
    }
}