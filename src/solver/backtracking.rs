use rand::{Rng, seq::SliceRandom};

use crate::board::{Board, Square};

#[derive(Debug, Clone, Copy)]
pub struct Map {
    pub big_squares: [Square; 9],
    pub cols: [Square; 9],
    pub rows: [Square; 9],
    pub filled: [Square; 9],
}
impl Default for Map {
    fn default() -> Self {
        Self {
            big_squares: [Square::all(); 9],
            cols: [Square::all(); 9],
            rows: [Square::all(); 9],
            filled: [Square::empty(); 9],
        }
    }
}
impl Map {
    pub fn generate(b: &Board) -> Self {
        let mut m = Self::default();
        for y in 0..9 {
            for x in 0..9 {
                if b.squares[y][x] != 0 {
                    m.erase(Square::from_number(b.squares[y][x]), x, y);
                }
            }
        }
        m
    }
    pub fn erase(&mut self, square: Square, x: usize, y: usize) {
        let square_x = x / 3;
        let square_y = y / 3;
        let i = square_y * 3 + square_x;

        self.big_squares[i].remove(square);
        self.cols[x].remove(square);
        self.rows[y].remove(square);
        self.filled[y] |= Square::from_x(x);
    }
    pub fn get(&self, x: usize, y: usize) -> Square {
        let square_x = x / 3;
        let square_y = y / 3;
        let i = square_y * 3 + square_x;
        if !self.filled[y].contains(Square::from_x(x)) {
            self.big_squares[i]
                .intersection(self.cols[x])
                .intersection(self.rows[y])
        } else {
            Square::empty()
        }
    }
}

pub fn count_solutions(b: Board) -> usize {
    count_solutions_map(b, Map::generate(&b))
}
fn count_solutions_map(mut b: Board, mut map: Map) -> usize {
    fill_in_singles(&mut b, &mut map);
    let (x, y) = find_nonempty(&map);
    if x == usize::MAX {
        return b.is_filled() as usize;
    }
    let mut solutions = 0;
    let n = map.get(x, y);
    for square in n.iter() {
        let mut cpy = b;
        let mut map_cpy = map;
        cpy.squares[y][x] = square.bits().trailing_zeros() as u8 + 1;
        map_cpy.erase(square, x, y);
        solutions += count_solutions_map(cpy, map_cpy);
    }
    solutions
}

pub fn count_branches(b: Board) -> usize {
    count_branches_map(b, Map::generate(&b))
}
fn count_branches_map(mut b: Board, mut map: Map) -> usize {
    fill_in_singles(&mut b, &mut map);
    let (x, y) = find_nonempty(&map);
    if x == usize::MAX {
        return 1;
    }
    let mut branches = 0;
    let n = map.get(x, y);
    for square in n.iter() {
        let mut cpy = b;
        let mut map_cpy = map;
        cpy.squares[y][x] = square.bits().trailing_zeros() as u8 + 1;
        map_cpy.erase(square, x, y);
        branches += count_branches_map(cpy, map_cpy);
    }
    branches
}
pub fn count_branches_to_sol(b: Board) -> usize {
    count_branches_to_sol_map(b, Map::generate(&b)).0
}
fn count_branches_to_sol_map(mut b: Board, mut map: Map) -> (usize, bool) {
    fill_in_singles(&mut b, &mut map);
    let (x, y) = find_nonempty(&map);
    if x == usize::MAX {
        return (1, b.is_filled());
    }
    let mut branches = 0;
    let n = map.get(x, y);
    for square in n.iter() {
        let mut cpy = b;
        let mut map_cpy = map;
        cpy.squares[y][x] = square.bits().trailing_zeros() as u8 + 1;
        map_cpy.erase(square, x, y);
        let (branch, solved) = count_branches_to_sol_map(cpy, map_cpy);
        branches += branch;
        if solved {
            return (branches, true);
        }
    }
    (branches, false)
}

pub fn solve(b: Board) -> Board {
    let map = Map::generate(&b);
    solve_map(b, map).0
}
fn solve_map(mut b: Board, mut map: Map) -> (Board, bool) {
    fill_in_singles(&mut b, &mut map);
    let (x, y) = find_nonempty(&map);
    if x == usize::MAX {
        return (b, b.is_filled());
    }
    let n = map.get(x, y);
    for square in n.iter() {
        let mut cpy = b;
        let mut map_cpy = map;
        cpy.squares[y][x] = square.bits().trailing_zeros() as u8 + 1;
        map_cpy.erase(square, x, y);
        let new = solve_map(cpy, map_cpy);
        if new.1 {
            return new;
        }
    }
    (b, false)
}
pub fn solve_map_random(mut b: Board, mut map: Map, rng: &mut impl Rng) -> (Board, bool) {
    fill_in_singles(&mut b, &mut map);
    let (x, y) = find_nonempty(&map);
    if x == usize::MAX {
        return (b, b.is_filled());
    }
    let n = map.get(x, y);
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    arr.shuffle(rng);

    for i in arr {
        let square = Square::from_number(i);
        if !n.contains(square) {
            continue;
        }
        let mut cpy = b;
        let mut map_cpy = map;
        cpy.squares[y][x] = square.bits().trailing_zeros() as u8 + 1;
        map_cpy.erase(square, x, y);
        let new = solve_map_random(cpy, map_cpy, rng);
        if new.1 {
            return new;
        }
    }
    (b, false)
}
fn fill_in_singles(b: &mut Board, map: &mut Map) {
    loop {
        let mut changed = false;
        for y in 0..9 {
            for x in 0..9 {
                let n = map.get(x, y);
                if n.bits().count_ones() == 1 && b.squares[y][x] == 0 {
                    b.squares[y][x] = n.bits().trailing_zeros() as u8 + 1;
                    map.erase(n, x, y);
                    changed = true;
                }
            }
        }
        if changed {
            continue;
        }
        let mut pos = [0; 9];
        for y in 0..9 {
            let mut counts = [0; 9];
            for x in 0..9 {
                let n = map.get(x, y);
                for s in n.iter() {
                    let num = s.bits().trailing_zeros() as usize;
                    counts[num] += 1;
                    pos[num] = x;
                }
            }
            for num in 0..9 {
                if counts[num] == 1 {
                    b.squares[y][pos[num]] = num as u8 + 1;
                    map.erase(Square::from_x(num), pos[num], y);
                    changed = true;
                }
            }
        }
        if changed {
            continue;
        }
        for x in 0..9 {
            let mut counts = [0; 9];
            for y in 0..9 {
                let n = map.get(x, y);
                for s in n.iter() {
                    let num = s.bits().trailing_zeros() as usize;
                    counts[num] += 1;
                    pos[num] = y;
                }
            }
            for num in 0..9 {
                if counts[num] == 1 {
                    b.squares[pos[num]][x] = num as u8 + 1;
                    map.erase(Square::from_x(num), x, pos[num]);
                    changed = true;
                }
            }
        }
        if changed {
            continue;
        }
        return;
    }
}
fn find_nonempty(map: &Map) -> (usize, usize) {
    let mut min = u32::MAX;
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    for y in 0..9 {
        for x in 0..9 {
            let n = map.get(x, y);
            if n.bits() == 0 {
                continue;
            }
            let count_ones = n.bits().count_ones();
            if count_ones < min {
                min_x = x;
                min_y = y;
                min = count_ones;
            }
        }
    }
    (min_x, min_y)
}
