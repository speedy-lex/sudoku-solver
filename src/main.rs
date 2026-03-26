use sudoku_solver::{generator, solver};

fn main() {
    let b = generator::generate();
    println!("{b}\n{b:?}");
    let b = solver::backtracking::solve(b);
    println!("{b}\n{b:?}");
}
