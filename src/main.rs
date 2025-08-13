use sudoku_solver::{solver, Board};

use std::str::FromStr;

fn main() {
    let b = Board::from_str("1----7-9--3--2---8--96--5----53--9---1--8---26----4---3------1--4------7--7---3--").unwrap();
    println!("{b}");
    let b = solver::solve(b);
    println!("{b}: {}", b.is_solved());
}