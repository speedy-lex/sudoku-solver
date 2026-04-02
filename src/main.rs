use std::{num::NonZero, str::FromStr, sync::{Arc, atomic::{AtomicUsize, Ordering}}, thread::{self, available_parallelism}, time::Duration};

use sudoku_solver::{Board, generator, solver::backtracking};

fn main() {
    println!("{:?}", backtracking::solve(Board::from_str("8----------36------7--9-2---5---7-------457-----1---3---1----68--85---1--9----4--").unwrap()));

    let threads = available_parallelism().unwrap_or(NonZero::new(1).unwrap()).get();

    let sudokus = Arc::new(AtomicUsize::new(0));

    for _ in 0..threads {
        let sudokus_ref = sudokus.clone();
        thread::spawn(move || {
            loop {
                let sudoku = generator::generate();
                let clues = sudoku.squares.as_flattened().iter().filter(|x| **x != 0).count();
                if clues <= 20 {
                    println!("{sudoku:?} {clues}");
                }
                sudokus_ref.fetch_add(1, Ordering::Relaxed);
            }
        });
    }
    loop {
        std::thread::sleep(Duration::from_secs(5));
        println!("{} sudokus generated", sudokus.load(Ordering::Relaxed));
    }
}
