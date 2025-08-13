use std::str::FromStr;

use sudoku_solver::{board::Board, solver::backtracking::{self, Map}};

fn main() {
    divan::main();
}

#[divan::bench]
fn solve() -> Board {
    backtracking::solve(Board::from_str("-8------9--12--3-87--------4------7-2-8--5------79-8----95-4----149-6------18--5-").unwrap())
}
#[divan::bench]
fn solve_ai_escargot() -> Board {
    backtracking::solve(Board::from_str("1----7-9--3--2---8--96--5----53--9---1--8---26----4---3------1--4------7--7---3--").unwrap())
}
#[divan::bench]
fn generate_map() -> Map {
    let b = Board::from_str("-8------9--12--3-87--------4------7-2-8--5------79-8----95-4----149-6------18--5-").unwrap();
    Map::generate(&b)
}