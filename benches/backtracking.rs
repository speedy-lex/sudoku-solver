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
fn solve_arto_inkala() -> Board {
    backtracking::solve(Board::from_str("8----------36------7--9-2---5---7-------457-----1---3---1----68--85---1--9----4--").unwrap())
}
#[divan::bench]
fn solve_unsolvable_49() -> Board {
    backtracking::solve(Board::from_str("..28......3..6...71......4.6...9.....5.6....9....57.6....3..1...7...6..84......2.").unwrap())
}
#[divan::bench]
fn solve_unsolvable_28() -> Board {
    backtracking::solve(Board::from_str("6....894.9....61...7..4....2..61..........2...89..2.......6...5.......3.8....16..").unwrap())
}
#[divan::bench]
fn solve_empty() -> Board {
    backtracking::solve(Board::empty())
}
#[divan::bench]
fn solve_easy() -> Board {
    backtracking::solve(Board::from_str("--95---------39--627-64-----2-975---1-746---23-61827----5-9462---4821-3---27-61--").unwrap())
}
#[divan::bench]
fn count_solutions() -> usize {
    backtracking::count_solutions(Board::from_str("--2----87--9---1---3--5-2-9-48---91----5-6-2--2--8-3-5-8--72---3618-5-9-2746-1853").unwrap())
}
#[divan::bench]
fn generate_map() -> Map {
    let b = Board::from_str("-8------9--12--3-87--------4------7-2-8--5------79-8----95-4----149-6------18--5-").unwrap();
    Map::generate(&b)
}
