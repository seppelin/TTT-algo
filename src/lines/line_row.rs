use crate::{BoardPos, DIRECTIONS, WIN_LEN};
use std::cmp::{max, min};

const M_LEN: u8 = WIN_LEN as u8- 1; // The len of Rows which have to be counted

trait GetValue{
    fn get_value(&self) -> i16;
}

impl GetValue for u8 {
    fn get_value(&self) -> i16 {
        *self as i16 * *self as i16
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Line{
    p2: BoardPos,
    p1: BoardPos,
    active: LineActive,
    sign: bool,
    len: u8
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum LineActive{
    ONE,
    BOTH
}

#[derive(Debug)]
pub(crate) struct LineRow{
	lines: Vec<Line>
}

impl LineRow {
    pub(super) fn new() -> Self{
    	LineRow { lines: Vec::new() }
    }

    pub(crate) fn print(&self){
        for i in &self.lines{
            println!("{:?}", i)
        }
    }

    fn get(&mut self, i: usize) -> &mut Line{ &mut self.lines[i] }

    // rows: the ones for the four directions of the point
    pub(super) unsafe fn update_rows(rows: [*mut LineRow; 4], pos: BoardPos, sign: bool) -> Option<Vec<(usize, i16)>>{
        // Value of pos is already removed!
        let mut changes = Vec::new();
        enum State{
            NONE,
            P1(usize),
            P2(usize),
            BOTH(usize, usize)
        }
        for dir in 0..4{
            let row = &mut *rows[dir];

            let mut state = State::NONE;
            for i in 0..row.lines.len(){
                if row.lines[i].p1 == pos{
                    state = match state {
                        State::NONE => State::P1(i),
                        State::P2(i2) => State::BOTH(i, i2),
                        _ => panic!("Two Lines have the same p1! Logic error!"),
                    };
                }
                if row.lines[i].p2 == pos{
                    state = match state {
                        State::NONE => State::P2(i),
                        State::P1(i2) => State::BOTH(i2, i),
                        _ => panic!("Two Lines have the same p2! Logic error!"),
                    };
                }
            }

            if match state{
                State::NONE => { Self::none(dir, pos, sign, row, &mut changes); false }

                State::P1(i1) => Self::p1(dir, pos, sign, i1, row, &mut changes),

                State::P2(i2) => Self::p2(dir, pos, sign, i2, row, &mut changes),

                State::BOTH(i1, i2) => Self::both(sign, i1, i2, row, &mut changes)
            }{
                return None;
            }
        }
        Some(changes)
    }

    fn none(dir: usize, pos: BoardPos, sign: bool, row: &mut LineRow, changes: &mut Vec<(usize, i16)>){
        let p1 = DIRECTIONS[dir*2] + pos;
        let p2 = DIRECTIONS[dir*2+1] + pos;

        // If the points are not in the field
        let active = if !p1.is_valid(){
            if !p2.is_valid(){
                return; // Both points are invalid -> not adding the line
            }
            changes.push((p2.get_usize(), 1));
            LineActive::ONE
        }
        else if !p2.is_valid(){
            changes.push((p1.get_usize(), 1));
            LineActive::ONE
        }
        else{
            changes.push((p1.get_usize(), 2));
            changes.push((p2.get_usize(), 2));
            LineActive::BOTH
        };

        row.lines.push(Line { p1, p2, active, sign, len: 1 });
    }

    fn p1(dir: usize, pos: BoardPos, sign: bool, i1: usize, row: &mut LineRow, changes: &mut Vec<(usize, i16)>) -> bool{
        let l1 = row.get(i1);

        // Making one long line
        if l1.sign == sign{
            if l1.len == M_LEN { return true; }
            l1.p1 = DIRECTIONS[dir*2] + pos;

            // Only active at pos
            if l1.active == LineActive::ONE{
                if l1.p1.is_valid(){
                    l1.len += 1;
                    changes.push((l1.p1.get_usize(), l1.len.get_value()))
                }
                else{
                    row.lines.remove(i1);
                }
            }
            // Both active
            else{
                changes.push((l1.p2.get_usize(), -2*l1.len.get_value()));

                if l1.p1.is_valid(){
                    l1.len += 1;
                    changes.push((l1.p1.get_usize(), 2*l1.len.get_value()));
                    changes.push((l1.p2.get_usize(), 2*l1.len.get_value()));
                }
                else{
                    l1.active = LineActive::ONE;
                    l1.len += 1;
                    changes.push((l1.p2.get_usize(), l1.len.get_value()));
                }
            }
        }

        // Ending the line at pos
        else{
            if l1.active == LineActive::ONE{
                row.lines.remove(i1);
            }
            else{
                changes.push((l1.p2.get_usize(), -l1.len.get_value()));
                l1.active = LineActive::ONE;
            }

            // If next to pos is valid adding line to pos
            let p1 = pos + DIRECTIONS[dir*2];
            if p1.is_valid() {
                changes.push((p1.get_usize(), 1));
                row.lines.push(Line { 
                    p1,
                    p2: pos + DIRECTIONS[dir*2+1],
                    active: LineActive::ONE,
                    sign,
                    len: 1 
                });
            }
        }
        false
    }

    fn p2(dir: usize, pos: BoardPos, sign: bool, i2: usize, row: &mut LineRow, changes: &mut Vec<(usize, i16)>) -> bool{
        let l2 = row.get(i2);

        // Making one long line
        if l2.sign == sign{
            if l2.len == M_LEN { return true; }
            l2.p2 = DIRECTIONS[dir*2+1] + pos;

            // Only active at pos
            if l2.active == LineActive::ONE{
                if l2.p2.is_valid(){
                    l2.len += 1;
                    changes.push((l2.p2.get_usize(), l2.len.get_value()))
                }
                else{
                    row.lines.remove(i2);
                }
            }
            // Both active
            else{
                changes.push((l2.p1.get_usize(), -2*l2.len.get_value()));

                if l2.p2.is_valid(){
                    l2.len += 1;
                    changes.push((l2.p1.get_usize(), 2*l2.len.get_value()));
                    changes.push((l2.p2.get_usize(), 2*l2.len.get_value()));
                }
                else{
                    l2.active = LineActive::ONE;
                    l2.len += 1;
                    changes.push((l2.p1.get_usize(), l2.len.get_value()));
                }
            }
        }

        // Ending the line at pos
        else{
            if l2.active == LineActive::ONE{
                row.lines.remove(i2);
            }
            else{
                changes.push((l2.p2.get_usize(), -l2.len.get_value()));
                l2.active = LineActive::ONE;
            }

            // If next to pos is valid adding line to pos
            let p2 = pos + DIRECTIONS[dir*2+1];
            if p2.is_valid() {
                changes.push((p2.get_usize(), 1));
                row.lines.push(Line { 
                    p1: pos + DIRECTIONS[dir*2],
                    p2,
                    active: LineActive::ONE,
                    sign,
                    len: 1 
                });
            }
        }
        false
    }

    fn both(sign: bool, i1: usize, i2: usize, row: &mut LineRow, changes: &mut Vec<(usize, i16)>) -> bool{
        // This is safe because in update_rows is garantueed that i1 != 12 and i am removing lines only at the end of the funcion
        let l2 = &mut unsafe{ *(row.lines.get_unchecked_mut(i2)) };
        let l1 = &mut unsafe{ *(row.lines.get_unchecked_mut(i1)) };

        if l1.sign == sign{
            if l2.sign == sign{
                if l1.active == LineActive::ONE{
                    // -|O 0 O|- remove lines
                    if l2.active == LineActive::ONE{
                        if l1.len + l2.len + 1 > M_LEN { return true; }
                        row.lines.remove(max(i1, i2));
                        row.lines.remove(min(i1, i2));
                    }
                    // -|O 0 O- L1 gets longer
                    else{
                        if l1.len + l2.len + 1 > M_LEN { return true; }
                        changes.push((l2.p1.get_usize(), -2*l2.len.get_value()));

                        l1.len += l2.len + 1;
                        l1.p1 = l2.p1;

                        changes.push((l1.p1.get_usize(), l1.len.get_value()));

                        row.lines.remove(i2);
                    }
                }
                else{
                    // -O 0 O|- L2 gets longer
                    if l2.active == LineActive::ONE{
                        if l2.len + l1.len + 1 > M_LEN { return true; }
                        changes.push((l1.p2.get_usize(), -2*l1.len.get_value()));

                        l2.len += l1.len + 1;
                        l2.p2 = l1.p2;

                        changes.push((l2.p2.get_usize(), l2.len.get_value()));

                        row.lines.remove(i1);
                    }
                    // -O0O- One long line
                    else{
                        if l1.len + l2.len + 1 > M_LEN { return true; }
                        changes.push((l1.p2.get_usize(), -2*l1.len.get_value()));
                        changes.push((l2.p1.get_usize(), -2*l2.len.get_value()));

                        l1.p1 = l2.p1;
                        l1.len += l2.len + 1;

                        changes.push((l1.p1.get_usize(), 2*l1.len.get_value()));
                        changes.push((l1.p2.get_usize(), 2*l1.len.get_value()));
                    }
                }
            }
            else{
                if l1.active == LineActive::ONE{
                    // -|O 0 X|- removing both
                    if l2.active == LineActive::ONE{
                        if l1.len == M_LEN { return true; }

                        row.lines.remove(max(i1, i2));
                        row.lines.remove(min(i1, i2));
                    }
                    // -|O 0 X- removing L1; update L2
                    else{
                        if l1.len == M_LEN { return true; }

                        l2.active = LineActive::ONE;
                        changes.push((l2.p1.get_usize(), -l2.len.get_value()));

                        row.lines.remove(i1);
                    }
                }
                else{
                    // -O 0 X|- update L1; remove L2
                    if l2.active == LineActive::ONE{
                        if l1.len == M_LEN { return true; }

                        changes.push((l1.p2.get_usize(), -2*l1.len.get_value()));

                        l1.active = LineActive::ONE;
                        l1.len += 1;

                        changes.push((l1.p2.get_usize(), l1.len.get_value()));

                        row.lines.remove(i2);
                    }
                    // -O 0 X- update both
                    else{
                        if l1.len == M_LEN { return true; }

                        changes.push((l2.p1.get_usize(), -l2.len.get_value()));
                        changes.push((l1.p2.get_usize(), -2*l1.len.get_value()));

                        l2.active = LineActive::ONE;
                        l1.active = LineActive::ONE;
                        l1.len += 1;

                        changes.push((l1.p2.get_usize(), l1.len.get_value()));
                    }
                }
            }
        }
        else{
            if l2.sign == sign{
                if l1.active == LineActive::ONE{
                    // -|X 0 O|- removing both
                    if l2.active == LineActive::ONE{
                        if l2.len == M_LEN { return true; }

                        row.lines.remove(max(i1, i2));
                        row.lines.remove(min(i1, i2));
                    }
                    // -|X 0 O- removing L1; update L2
                    else{
                        if l2.len == M_LEN { return true; }

                        changes.push((l2.p1.get_usize(), -2*l2.len.get_value()));

                        l2.active = LineActive::ONE;
                        l2.len += 1;

                        changes.push((l2.p1.get_usize(), l2.len.get_value()));

                        row.lines.remove(i1);
                    }
                }
                else{
                    // -X 0 O|- update L1; remove L2
                    if l2.active == LineActive::ONE{
                        if l2.len == M_LEN { return true; }

                        l1.active = LineActive::ONE;
                        changes.push((l1.p2.get_usize(), -l1.len.get_value()));

                        row.lines.remove(i2);
                    }
                    // -X 0 O- update both
                    else{
                        if l2.len == M_LEN { return true; }

                        changes.push((l2.p1.get_usize(), -2*l2.len.get_value()));
                        changes.push((l1.p2.get_usize(), -l1.len.get_value()));

                        l1.active = LineActive::ONE;
                        l2.active = LineActive::ONE;
                        l2.len += 1;

                        changes.push((l2.p1.get_usize(), l2.len.get_value()));
                    }
                }
            }
            else{
                if l1.active == LineActive::ONE{
                    // -|X 0 X|- removing both
                    if l2.active == LineActive::ONE{
                        row.lines.remove(max(i1, i2));
                        row.lines.remove(min(i1, i2));
                    }
                    // -|X 0 X- removing L1; update L2
                    else{
                        l2.active = LineActive::ONE;
                        changes.push((l2.p1.get_usize(), -l2.len.get_value()));

                        row.lines.remove(i1);
                    }
                }
                else{
                    // -X 0 X|- update L1; remove L2
                    if l2.active == LineActive::ONE{
                        l1.active = LineActive::ONE;
                        changes.push((l1.p2.get_usize(), -l1.len.get_value()));

                        row.lines.remove(i2);
                    }
                    // -X 0 X- update both
                    else{
                        changes.push((l1.p2.get_usize(), -l1.len.get_value()));
                        changes.push((l2.p1.get_usize(), -l2.len.get_value()));

                        l1.active = LineActive::ONE;
                        l2.active = LineActive::ONE;
                    }
                }
            }
        }
        false
    }
}


impl Clone for LineRow {
    fn clone(&self) -> Self{
    	let mut clone = Vec::with_capacity(self.lines.capacity());
    	for i in &self.lines{
    		clone.push(i.clone());
    	}
    	LineRow{ lines: clone }
    } 
}
