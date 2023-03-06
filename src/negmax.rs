use crate::{WIDTH, HEIGHT, MAX_SCORE, MIN_SCORE, lines::{Lines, UpdateResult}, BoardPos, MAX_DEPTH};
use std::cmp::max;

const fn get_moves() -> [BoardPos; WIDTH*HEIGHT]{
	let mut x = [BoardPos::zero(); WIDTH*HEIGHT];
	let mut i = 0;
	while i != WIDTH*HEIGHT{
		x[i].h = (i%HEIGHT) as i8;
        x[i].w = (i / HEIGHT) as i8;
		i += 1;
	}
	x
}

pub(crate) fn negmax(
	board: &mut [u8; WIDTH*HEIGHT],
	sign: bool,
	count: isize,
    depth: usize,
	mut alpha: isize,
    mut beta: isize,
	lines: &mut Lines) -> isize
{
	if count == (WIDTH*HEIGHT-1) as isize { return 0; }
    let current_max_score = MAX_SCORE - count;
    if current_max_score <= beta{
        beta = current_max_score;
        if alpha >=  beta { return beta; }
    }
    if depth == MAX_DEPTH { return 0; }

	let moves = sorted_moves(lines.get_values());
    
    let mut best_score = MIN_SCORE;

    for m in moves{
        if board[m.get_usize()] != 0{ continue; }
        match lines.update_move(m, sign){
            UpdateResult::Continue(changes) => {
                board[m.get_usize()] = sign as u8 + 1;

                best_score = max(best_score, -negmax(board, !sign, count + 1, depth + 1, -beta, -alpha, lines));

                lines.redo_changes(changes);
                board[m.get_usize()] = 0;

                alpha = max(alpha, best_score);
                if alpha >= beta{ break; }
            }
            UpdateResult::Win(changes) => {
                lines.redo_changes(changes);
                return current_max_score
            }
        }
    }
	best_score
}

pub(crate) fn sorted_moves(values: &[i16; WIDTH*HEIGHT]) -> [BoardPos; WIDTH*HEIGHT]{
    let mut moves = get_moves();
    let mut nums = 0;

    for i in 0..WIDTH*HEIGHT{
        if values[moves[i].get_usize()] != 0{
            moves.swap(nums, i);
            nums += 1;
        }
    }
    moves[..nums].sort_unstable_by(|a, b| values[b.get_usize()].cmp(&values[a.get_usize()]));

    moves
}