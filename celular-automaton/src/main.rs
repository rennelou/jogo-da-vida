use ndarray::{arr2};
use automaton::{Grid, Boundary};
use std::process;

fn main() {
    let mut states = arr2(&[[0, 0, 0, 0, 0],
                            [0, 0, 0, 0, 0],
                            [0, 0, 1, 1, 1],
                            [0, 0, 1, 0, 0],
                            [0, 0, 0, 1, 0]]);

    let grid = Grid::new(&states, Boundary::Limited).unwrap_or_else(|err| {
        eprintln!("Não foi possível a criação do grid: {}", err);
        process::exit(1);
    });
    
    println!("t0\n{}", states);
    for tick in 1..6 {
        states = grid.tick(&states);
        println!("t{}\n{}", tick, states);
    }
}