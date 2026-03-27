use std::{fs, str::FromStr};

use sudoku_solver::{Board, solver::backtracking};

fn main() {
    let sudokus = String::from_utf8(fs::read("4096sudokus").unwrap()).unwrap();
    for sudoku in sudokus.lines() {
        let b = Board::from_str(sudoku).unwrap();
        println!("{}  {}/{}", sudoku, backtracking::count_branches_to_sol(b), backtracking::count_branches(b));
    }
}
