use std::{fmt::Display, str::FromStr};

use bitflags::bitflags;

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub squares: [[u8; 9]; 9]
}
impl Board {
    pub fn empty() -> Self {
        Self { squares: [[0; 9]; 9] }
    }
    pub fn is_filled(&self) -> bool {
        !self.squares.iter().flat_map(|x| x.iter()).any(|x| *x == 0)
    }
    pub fn is_solved(&self) -> bool {
        if !self.is_filled() {
            return false;
        }
        let mut rows = [Square::empty(); 9];
        let mut cols = [Square::empty(); 9];
        let mut squares = [Square::empty(); 9];
        for (i, row) in self.squares.iter().zip(rows.iter_mut()) {
            for cell in i {
                row.insert(Square::from_number(*cell));
            }
        }
        for row in self.squares.iter() {
            for (i, cell) in row.iter().enumerate() {
                cols[i].insert(Square::from_number(*cell));
            }
        }
        for square_y in 0..3 {
            for square_x in 0..3 {
                let i = square_y * 3 + square_x;
                let square = &mut squares[i];
                for y in (square_y * 3)..(square_y * 3 + 3) {
                    for x in (square_x * 3)..(square_x * 3 + 3) {
                        square.insert(Square::from_number(self.squares[y][x]));
                    }
                }
            }
        }
        rows.into_iter().all(|x| x.is_all()) &&
        cols.into_iter().all(|x| x.is_all()) &&
        squares.into_iter().all(|x| x.is_all())
    }
    pub fn dump(&self) -> String {
        let mut str = String::with_capacity(81);
        for x in self.squares.as_slice().as_flattened() {
            str.push(match *x {
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => '-',
            });
        }
        str
    }
}
impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Self::empty();
        let mut i = 0;
        for ch in s.chars() {
            let x = match ch {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                '-' => 0,
                _ => continue,
            };
            board.squares.as_flattened_mut()[i] = x;
            i += 1;
            if i == 81 {
                return Ok(board);
            }
        }
        Err(())
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.squares.as_chunks::<3>().0 {
            writeln!(f, "+---+---+---+")?;
            for row in row {
                for chunk in row.as_chunks::<3>().0 {
                    write!(f, "|")?;
                    for x in chunk {
                        if *x == 0 {
                            write!(f, "-")?;
                        } else {
                            write!(f, "{x}")?;
                        }
                    }
                }
                writeln!(f, "|")?;
            }
        }
        write!(f, "+---+---+---+")?;
        Ok(())
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Square: u16 {
        const N1 =   1;
        const N2 =   2;
        const N3 =   4;
        const N4 =   8;
        const N5 =  16;
        const N6 =  32;
        const N7 =  64;
        const N8 = 128;
        const N9 = 256;
    }
}
impl Square {
    pub fn from_number(x: u8) -> Self {
        Self::from_bits_retain(1 << (x - 1))
    }
}
