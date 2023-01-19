mod lines;

use lines::*;

pub(crate) const DIRECTIONS: [AlgoPos; 8] =
[
    AlgoPos{ w: -1, h: 0 }, AlgoPos{ w: 1, h: 0 }, // Left; right
    AlgoPos{ w: 0, h: -1 }, AlgoPos{ w: 0, h: 1 }, // Up; down
    AlgoPos{ w: -1, h: -1 }, AlgoPos{ w: 1, h: 1 }, // Left up; right down
    AlgoPos{ w: 1, h: -1 }, AlgoPos{ w: -1, h: 1 }, // Right up; left down
];

pub(crate) const GREEN: u8 = 1;
pub(crate) const RED: u8 = 2;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct AlgoPos{
    pub w: i8,
    pub h: i8,
}

impl AlgoPos {
    pub(crate) fn is_valid(&self) -> bool{
        if self.w < 0 || self.w > 19 || self.h < 0 || self.h > 19{
            return false;
        }
        true
    }
}

impl std::ops::Add for AlgoPos {
    type Output = AlgoPos;

    fn add(self, rhs: Self) -> Self::Output {
        AlgoPos{ w: self.w + rhs.w, h: self.h + rhs.h }
    }
}

pub struct Algo{
    lines: Lines,
    board: [[u8; 20]; 20]
}

impl Algo {
    pub fn empty() -> Self{
        Self { lines: Lines::new(), board: [[0; 20]; 20] }
    }

    pub fn from_board(board: &[[u8; 20]; 20]) -> Self{
        Self { lines: Lines::new(), board: board.clone() }
    }

    pub fn calculate(&self, next_sign: bool) -> (usize, usize){
        (0, 0)
    }

    pub fn make_move(&mut self, pos: (usize, usize), sign: bool) -> bool{
        let algo_pos = AlgoPos{ w: pos.0 as i8, h: pos.1 as i8 };
        
        if self.board[pos.0][pos.1] != 0{
            return false;
        }

        self.board[pos.0][pos.1] = sign as u8 + 1;
        self.lines.update_move(algo_pos, sign);
        true
    }

    pub fn print_board(&self){
        for h in 0..20{
            print!("|");
            for w in 0.. 20{
                match self.board[w][h] {
                    0 => print!(" |"),
                    1 => print!("x|"),
                    2 => print!("o|"),
                    _ => panic!("logic error!")
                }
            }
            println!("")
        }
    }

    pub fn print_lines(&self){
        self.lines.print();
    }
}
