use sudoku_solver::{Board, generator, solver};

use std::str::FromStr;

fn main() {
    let b = generator::generate();
    println!("{b}\n{b:?}");
    let b = solver::backtracking::solve(b);
    println!("{b}\n{b:?}");
}
