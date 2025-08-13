use std::str::FromStr;

use sudoku_solver::Board;

fn main() {
    divan::main();
}

#[divan::bench]
fn from_str() -> Board {
    Board::from_str("-8------9--12--3-87--------4------7-2-8--5------79-8----95-4----149-6------18--5-").unwrap()
}