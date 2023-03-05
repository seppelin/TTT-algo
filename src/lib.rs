mod lines;
mod negmax;

use lines::*;
use negmax::*;

pub(crate) const DIRECTIONS: [BoardPos; 8] =
[
    BoardPos{ w: 1, h: 0 }, BoardPos{ w: -1, h: 0 }, // right; left
    BoardPos{ w: 0, h: 1 }, BoardPos{ w: 0, h: -1 }, // down; up
    BoardPos{ w: 1, h: 1 }, BoardPos{ w: -1, h: -1 }, // right-down; left-up
    BoardPos{ w: 1, h: -1 }, BoardPos{ w: -1, h: 1 }, // Right up; left down
];

pub const WIDTH: usize = 4;
pub const HEIGHT: usize = 4;
pub const WIN_LEN: usize = 4;
pub const MAX_SCORE: isize = (WIDTH*HEIGHT+1) as isize;
pub const MIN_SCORE: isize = -((WIDTH*HEIGHT+1) as isize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BoardPos{
    pub w: i8,
    pub h: i8,
}

impl BoardPos {
    pub(crate) fn is_valid(&self) -> bool{
        if self.w < 0 || self.w >= WIDTH as i8 || self.h < 0 || self.h >= HEIGHT as i8{
            return false;
        }
        true
    }

    pub(crate) fn get_usize(&self) -> usize{
        return self.w as usize*HEIGHT + self.h as usize;
    }

    pub(crate) const fn zero() -> Self{
        return Self{ h: 0, w: 0};
    }
}

impl std::ops::Add for BoardPos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        BoardPos{ w: self.w + rhs.w, h: self.h + rhs.h }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AlgoState {
    InGame,
    Draw,
    Won(bool)
}

pub struct Algo{
    state: AlgoState,
    board: [u8; WIDTH*HEIGHT],
    count: isize,
    lines: Lines,
}

impl Algo {
    pub fn empty() -> Self{
        Self { state: AlgoState::InGame, board: [0; WIDTH*HEIGHT], count: 0, lines: Lines::new() }
    }

    pub fn get_state(&self) -> &AlgoState{ return &self.state; }

    pub fn calculate(&self, next_sign: bool) -> Option<(BoardPos, isize)>{
        match self.state {
            AlgoState::InGame => (),
            _ => return None,
        }
        let mut alpha = MIN_SCORE;
        let beta = MAX_SCORE;
        let mut board = self.board.clone();
        let mut lines = self.lines.clone();

        let moves = sorted_moves(&self.lines.get_values());
        let mut best_score = MIN_SCORE;
        let mut best_move = BoardPos::zero();

        for m in moves{
            if board[m.get_usize()] != 0{ continue; }
            match lines.update_move(m, next_sign){
                UpdateResult::Continue(changes) => {
                    board[m.get_usize()] = next_sign as u8 + 1;
                    let score = -negmax(&mut board, !next_sign, self.count + 1, -beta, -alpha, &mut lines);

                    lines.redo_changes(changes);
                    board[m.get_usize()] = 0;

                    if score > best_score{
                        best_score = score; best_move = m;
                        alpha = std::cmp::max(alpha, best_score);
                        if alpha >= beta { break; }
                    }
                }
                UpdateResult::Win(_) => {
                    return Some((m, MAX_SCORE-self.count));
                }
            }
        }
        Some((best_move, best_score))
    }

    pub fn make_move(&mut self, pos: BoardPos, sign: bool) -> bool{
        match self.state {
            AlgoState::InGame => (),
            _ => return false,
        }
        
        if self.board[pos.get_usize()] != 0{
            return false;
        }

        self.board[pos.get_usize()] = sign as u8 + 1;
        self.count += 1;

        if self.count == (WIDTH*HEIGHT) as isize { self.state = AlgoState::Draw; }

        let result = self.lines.update_move(pos, sign);
        match result {
            UpdateResult::Win(_) => self.state = AlgoState::Won(sign),
            _ => ()
        }
        true
    }

    pub fn print(&self){
        self.print_board();
        self.print_values();
        self.print_lines();
    }

    pub fn print_board(&self){
        for h in 0..HEIGHT{
            print!("|");
            for w in 0.. WIDTH{
                match self.board[w*HEIGHT + h] {
                    0 => print!(" |"),
                    1 => print!("o|"),
                    2 => print!("x|"),
                    _ => panic!("logic error!")
                }
            }
            println!("")
        }
    }

    pub fn print_values(&self){
        let values = self.lines.get_values();
        for h in 0..HEIGHT{
            print!("|");
            for w in 0..WIDTH{
                print!("{}|", values[w*HEIGHT+h])
            }
            println!("")
        }
    }

    pub fn print_lines(&self){
        self.lines.print();
    }
}
