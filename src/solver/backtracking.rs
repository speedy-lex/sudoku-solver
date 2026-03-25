use crate::board::{Board, Square};


#[derive(Debug, Clone, Copy)]
pub struct Map {
    pub big_squares: [Square; 9],
    pub cols: [Square; 9],
    pub rows: [Square; 9],
    pub filled: [Square; 9],
}
impl Map {
    pub fn generate(b: &Board) -> Self {
        let mut m = Self { big_squares: [Square::all(); 9], cols: [Square::all(); 9], rows: [Square::all(); 9], filled: [Square::empty(); 9] };
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
            self.big_squares[i].intersection(self.cols[x]).intersection(self.rows[y])
        } else {
            Square::empty()
        }
    }
}

pub fn solve(b: Board) -> Board {
    let map = Map::generate(&b);
    solve_map(b, map).0
}
fn solve_map(mut b: Board, map: Map) -> (Board, bool) {
    for y in 0..9 {
        for x in 0..9 {
            let n = map.get(x, y);
            if n.bits().count_ones() == 1 && b.squares[y][x] == 0 {
                b.squares[y][x] = n.bits().trailing_zeros() as u8 + 1;
            }
        }
    }
    solve_branch(b, Map::generate(&b))
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
fn solve_branch(b: Board, map: Map) -> (Board, bool) {
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
