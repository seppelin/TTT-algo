use crate::{AlgoPos, DIRECTIONS};
use std::cmp::{max, min};

trait GetValue{
    fn get_value(&self) -> i16;
}

impl GetValue for u8 {
    fn get_value(&self) -> i16 {
        *self as i16 * *self as i16
    }
}

pub(crate) struct LineRow{
	lines: Vec<Line>
}

impl LineRow {
    pub(super) fn new() -> Self{
    	LineRow { lines: Vec::with_capacity(4) }
    }

    pub(super) fn capacity(c: usize) -> Self{
    	LineRow { lines: Vec::with_capacity(c) }
    }

    pub(crate) fn print(&self){
        for i in &self.lines{
            let state = match &i.active{
                LineActive::ONE => "One",
                LineActive::BOTH => "Both",
            };
            println!("Line{{ {} P1: {} {}, P2: {} {}, LEN: {}, Sign: {} }}", state, i.p1.w, i.p1.h,
                i.p2.w, i.p2.h, i.len, i.sign)
        }
    }

    pub(crate) fn get_lines(&mut self) -> &mut Vec<Line>{ &mut self.lines }

    fn get(&mut self, i: i8) -> &mut Line{ &mut self.lines[i as usize] }

    pub(super) unsafe fn update_rows(rows: [*mut LineRow; 4], pos: AlgoPos, sign: bool) -> Vec<(AlgoPos, i16)>{ // rows: the ones for the four directions of the point     
        // Value of pos is already removed!
        let mut changes = Vec::new();
        enum State{
            NONE,
            P1(i8),
            P2(i8),
            BOTH(i8, i8)
        }
        for dir in 0..4{
            let row = &mut *rows[dir];

            let mut state = State::NONE;
            for i in 0..row.lines.len(){
                if row.lines[i].p1 == pos{
                    state = match state {
                        State::NONE => State::P1(i as i8),
                        State::P2(i2) => State::BOTH(i as i8, i2),
                        _ => panic!("Two Lines have the same p1! Logic error!"),
                    };
                }
                if row.lines[i].p2 == pos{
                    state = match state {
                        State::NONE => State::P2(i as i8),
                        State::P1(i2) => State::BOTH(i2, i as i8),
                        _ => panic!("Two Lines have the same p2! Logic error!"),
                    };
                }
            }

            match state{
                State::NONE => Self::none(dir, pos, sign, row, &mut changes),

                State::P1(i1) => Self::p1(dir, pos, sign, i1, row, &mut changes),

                State::P2(i2) => Self::p2(dir, pos, sign, i2, row, &mut changes),

                State::BOTH(i1, i2) => Self::both(sign, i1, i2, row, &mut changes)
            }
        }
        changes
    }

    fn none(dir: usize, pos: AlgoPos, sign: bool, row: &mut LineRow, changes: &mut Vec<(AlgoPos, i16)>){
        let p1 = DIRECTIONS[dir*2] + pos;
        let p2 = DIRECTIONS[dir*2+1] + pos;

        // If the points are not in the field
        let active = if !p1.is_valid(){
            if !p2.is_valid(){
                return; // Both points are invalid -> not adding the line
            }
            changes.push((p2, 1));
            LineActive::ONE
        }
        else if !p2.is_valid(){
            changes.push((p1, 1));
            LineActive::ONE
        }
        else{
            changes.push((p1, 2));
            changes.push((p2, 2));
            LineActive::BOTH
        };

        row.lines.push(Line { p1, p2, active, sign, len: 1 });
    }

    fn p1(dir: usize, pos: AlgoPos, sign: bool, i1: i8, row: &mut LineRow, changes: &mut Vec<(AlgoPos, i16)>){
        let l1 = row.get(i1);

        // Making one long line
        if l1.sign == sign{
            l1.p1 = DIRECTIONS[dir*2] + pos;

            // Only active at pos
            if l1.active == LineActive::ONE{
                if l1.p1.is_valid(){
                    l1.len += 1;
                    changes.push((l1.p1, l1.len.get_value()))
                }
                else{
                    row.lines.remove(i1 as usize);
                }
            }
            else{
                changes.push((l1.p2, -2*l1.len.get_value()));

                if l1.p1.is_valid(){
                    l1.len += 1;
                    changes.push((l1.p1, 2*l1.len.get_value()));
                    changes.push((l1.p2, 2*l1.len.get_value()));
                }
                else{
                    l1.active = LineActive::ONE;
                    l1.len += 1;
                    changes.push((l1.p2, l1.len.get_value()));
                }
            }
        }

        // Ending the line at pos
        else{
            if l1.active == LineActive::ONE{
                row.lines.remove(i1 as usize);
            }
            else{
                changes.push((l1.p2, -l1.len.get_value()));
                l1.active = LineActive::ONE;
            }

            // If next to pos is valid adding line to pos
            let p1 = pos + DIRECTIONS[dir*2];
            if p1.is_valid() {
                changes.push((p1, 1));
                row.lines.push(Line { 
                    p1: p1,
                    p2: pos + DIRECTIONS[dir*2+1],
                    active: LineActive::ONE,
                    sign: sign,
                    len: 1 
                });
            }
        }
    }

    fn p2(dir: usize, pos: AlgoPos, sign: bool, i2: i8, row: &mut LineRow, changes: &mut Vec<(AlgoPos, i16)>){
        let l2 = row.get(i2);

        // Making one long line
        if l2.sign == sign{
            l2.p2 = DIRECTIONS[dir*2+1] + pos;

            // Only active at pos
            if l2.active == LineActive::ONE{
                if l2.p2.is_valid(){
                    l2.len += 1;
                    changes.push((l2.p2, l2.len.get_value()))
                }
                else{
                    row.lines.remove(i2 as usize);
                }
            }
            else{
                changes.push((l2.p1, -2*l2.len.get_value()));

                if l2.p2.is_valid(){
                    l2.len += 1;
                    changes.push((l2.p1, 2*l2.len.get_value()));
                    changes.push((l2.p2, 2*l2.len.get_value()));
                }
                else{
                    l2.active = LineActive::ONE;
                    l2.len += 1;
                    changes.push((l2.p1, l2.len.get_value()));
                }
            }
        }

        // Ending the line at pos
        else{
            if l2.active == LineActive::ONE{
                row.lines.remove(i2 as usize);
            }
            else{
                changes.push((l2.p2, -l2.len.get_value()));
                l2.active = LineActive::ONE;
            }

            // If next to pos is valid adding line to pos
            let p2 = pos + DIRECTIONS[dir*2+1];
            if p2.is_valid() {
                changes.push((p2, 1));
                row.lines.push(Line { 
                    p1: pos + DIRECTIONS[dir*2],
                    p2: p2,
                    active: LineActive::ONE,
                    sign: sign,
                    len: 1 
                });
            }
        }
    }

    fn both(sign: bool, i1: i8, i2: i8, row: &mut LineRow, changes: &mut Vec<(AlgoPos, i16)>){
        if row.get(i1).sign == sign{

            // Making one long line!
            if row.get(i2).sign == sign{
                let l2 = row.get(i2).clone();
                let l1 = row.get(i1);

                if l1.active == LineActive::ONE{
                    // Both had only pos active -> removing lines
                    if l2.active == LineActive::ONE{
                        row.lines.remove(max(i1 as usize, i2 as usize));
                        row.lines.remove(min(i1 as usize, i2 as usize));
                    }
                    // L2 has both active, L1 only pos -> L1 just gets longer
                    else{
                        changes.push((l2.p1, -2*l2.len.get_value()));

                        l1.len += l2.len + 1;
                        l1.p1 = l2.p1;

                        changes.push((l1.p1, l1.len.get_value()));

                        row.lines.remove(i2 as usize);
                    }
                }
                else{
                    // The new p1 is not active anymore -> ONE active
                    if l2.active == LineActive::ONE{
                        changes.push((l1.p2, -2*l1.len.get_value()));

                        l1.len += l2.len + 1;
                        l1.active = LineActive::ONE;
                        l1.p1 = l2.p1;

                        changes.push((l1.p2, l1.len.get_value()));

                        row.lines.remove(i2 as usize);
                    }
                    // L1 and L2 were BOTH active -> one long line, BOTH active
                    else{
                        changes.push((l1.p2, -2*l1.len.get_value()));
                        changes.push((l2.p1, -2*l2.len.get_value()));

                        l1.p1 = l2.p1;
                        l1.len += l2.len + 1;

                        changes.push((l1.p1, 2*l1.len.get_value()));
                        changes.push((l1.p2, 2*l1.len.get_value()));
                    }
                }
            }

            // Ending l2 at pos, l1 after pos
            else{
                if row.get(i1).active == LineActive::ONE{
                    // Both had only pos active -> removing them
                    if row.get(i2).active == LineActive::ONE{
                        row.lines.remove(max(i1 as usize, i2 as usize));
                        row.lines.remove(min(i1 as usize, i2 as usize));
                    }
                    // Removing l1, updating l2
                    else{
                        {
                            let l2 = row.get(i2);

                            l2.active = LineActive::ONE;
                            changes.push((l2.p1, -l2.len.get_value()));
                        }
                        row.lines.remove(i1 as usize);
                    }
                }
                else{
                    // Removing l2, updating l1 + adding pos
                    if row.get(i2).active == LineActive::ONE{
                        {
                            let l1 = row.get(i1);
                            changes.push((l1.p2, -2*l1.len.get_value()));

                            l1.active = LineActive::ONE;
                            l1.len += 1;

                            changes.push((l1.p2, l1.len.get_value()));
                        }
                        row.lines.remove(i2 as usize);
                    }
                    // Updating both, adding pos to l1
                    else{
                        {
                            let l2 = row.get(i2);

                            l2.active = LineActive::ONE;
                            changes.push((l2.p1, -l2.len.get_value()));
                        }
                        {
                            let l1 = row.get(i1);
                            changes.push((l1.p2, -2*l1.len.get_value()));

                            l1.active = LineActive::ONE;
                            l1.len += 1;

                            changes.push((l1.p2, l1.len.get_value()));
                        }
                    }
                }
            }
        }
        else{
            // Ending l1 at pos, l2 after pos
            if row.get(i2).sign == sign{
                if row.get(i1).active == LineActive::ONE{
                    // Both had only pos active -> removing them
                    if row.get(i2).active == LineActive::ONE{
                        row.lines.remove(max(i1 as usize, i2 as usize));
                        row.lines.remove(min(i1 as usize, i2 as usize));
                    }
                    
                    // Removing l1, updating l2 + adding pos
                    else{
                        {
                            let l2 = row.get(i2);
                            changes.push((l2.p1, -2*l2.len.get_value()));

                            l2.active = LineActive::ONE;
                            l2.len += 1;

                            changes.push((l2.p1, l2.len.get_value()));
                        }
                        row.lines.remove(i1 as usize);
                        
                    }
                }
                else{
                    // Removing l2, updating l1
                    if row.get(i2).active == LineActive::ONE{
                        {
                            let l1 = row.get(i1);

                            l1.active = LineActive::ONE;
                            changes.push((l1.p2, -l1.len.get_value()));
                        }
                        row.lines.remove(i2 as usize);
                    }
                    // Updating both, adding pos to l2
                    else{
                        {
                            let l2 = row.get(i2);
                            changes.push((l2.p1, -2*l2.len.get_value()));

                            l2.active = LineActive::ONE;
                            l2.len += 1;

                            changes.push((l2.p1, l2.len.get_value()));
                        }
                        {
                            let l1 = row.get(i1);

                            l1.active = LineActive::ONE;
                            changes.push((l1.p2, -l1.len.get_value()));
                        }
                    }
                }
            }
            // Ending both at pos
            else{
                if row.get(i1).active == LineActive::ONE{
                    // Both had only pos active -> removing them
                    if row.get(i2).active == LineActive::ONE{
                        row.lines.remove(max(i1 as usize, i2 as usize));
                        row.lines.remove(min(i1 as usize, i2 as usize));
                    }
                    // Removing l1, updating l2
                    else{
                        {
                            let l2 = row.get(i2);

                            l2.active = LineActive::ONE;
                            changes.push((l2.p1, -l2.len.get_value()));
                        }
                        row.lines.remove(i1 as usize);
                    }
                }
                else{
                    // Removing l2, updating l1
                    if row.get(i2).active == LineActive::ONE{
                        {
                            let l1 = row.get(i1);

                            l1.active = LineActive::ONE;
                            changes.push((l1.p2, -l1.len.get_value()));
                        }
                        row.lines.remove(i2 as usize);
                    }
                    // Updating both
                    else{
                        {
                            let l2 = row.get(i2);

                            l2.active = LineActive::ONE;
                            changes.push((l2.p1, -l2.len.get_value()));
                        }
                        {
                            let l1 = row.get(i1);

                            l1.active = LineActive::ONE;
                            changes.push((l1.p2, -l1.len.get_value()));
                        }
                    }
                }
            }
        }
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

#[derive(Clone)]
pub(crate) struct Line{
    p1: AlgoPos,
    p2: AlgoPos,
    active: LineActive,
    sign: bool,
    len: u8
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum LineActive{
    ONE,
    BOTH
}