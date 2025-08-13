use crate::board::{Board, Square};


#[derive(Debug, Clone, Copy)]
pub struct Map {
    pub squares: [[Square; 9]; 9],
}
impl Map {
    pub fn generate(b: &Board) -> Self {
        let mut m = Self { squares: [[Square::all(); 9]; 9]};
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

        self.squares[y][x] = Square::empty();
        for i in self.squares[y].iter_mut() {
            i.remove(square);
        }
        for i in self.squares.as_flattened_mut().iter_mut().skip(x).step_by(9) {
            i.remove(square);
        }
        for y in (square_y * 3)..(square_y * 3 + 3) {
            for x in (square_x * 3)..(square_x * 3 + 3) {
                self.squares[y][x].remove(square);
            }
        }
    }
}

pub fn solve(b: Board) -> Board {
    let map = Map::generate(&b);
    solve_map(b, map).0
}
fn solve_map(mut b: Board, mut map: Map) -> (Board, bool) {
    for y in 0..9 {
        for x in 0..9 {
            let n = map.squares[y][x];
            if n.bits().count_ones() == 1 {
                b.squares[y][x] = n.bits().trailing_zeros() as u8 + 1;
                map.erase(n, x, y);
            }
        }
    }
    solve_branch(b, map)
}
fn find_nonempty(map: &Map) -> (usize, usize) {
    let mut min = 9;
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    for y in 0..9 {
        for x in 0..9 {
            let n = map.squares[y][x];
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
    let n = map.squares[y][x];
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