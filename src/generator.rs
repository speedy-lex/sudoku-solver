use std::collections::HashSet;

use rand::seq::IteratorRandom;

use crate::{
    Board,
    solver::backtracking::{Map, count_solutions, solve_map_random},
};

pub fn generate() -> Board {
    let mut rng: rand::rngs::SmallRng = rand::make_rng();
    let mut b = solve_map_random(Board::empty(), Map::default(), &mut rng).0;
    let mut coords = HashSet::with_capacity(81);
    for x in 0..9 {
        for y in 0..9 {
            coords.insert((x, y));
        }
    }
    let mut valid = coords.clone();
    loop {
        let &(x, y) = valid.iter().choose(&mut rng).unwrap();
        let mut cpy = b;
        cpy.squares[y][x] = 0;
        if count_solutions(cpy) != 1 {
            valid.remove(&(x, y));
            if valid.is_empty() {
                return b;
            }
            continue;
        }
        b = cpy;
        coords.remove(&(x, y));
        valid = coords.clone();
    }
}

#[test]
fn test_solve_map_random() {
    let mut rng: rand::rngs::SmallRng = rand::make_rng();
    for _ in 0..128 {
        let b = solve_map_random(Board::empty(), Map::default(), &mut rng).0;
        assert!(b.is_solved());
    }
}
