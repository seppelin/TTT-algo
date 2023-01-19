use crate::{AlgoPos, DIRECTIONS};
use std::cmp::{max, min};

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
            let p1_state = match &i.active {
                LineActive::P2 => "off",
                _ => "on",
            };
            let p2_state = match &i.active {
                LineActive::P1 => "off",
                _ => "on",
            };
            println!("Line{{ P1: {} {} {}, P2: {} {} {}, LEN: {}, Sign: {} }}", i.p1.w, i.p1.h, p1_state,
                i.p2.w, i.p2.h, p2_state, i.len, i.sign)
        }
    }

    pub(crate) fn get_lines(&mut self) -> &mut Vec<Line>{ &mut self.lines }

    fn get(&mut self, i: i8) -> &mut Line{
        &mut self.lines[i as usize]
    }

    pub(super) unsafe fn update_rows(rows: [*mut LineRow; 4], pos: AlgoPos, sign: bool){ // rows: the ones for the four directions of the point     
        enum State{
            NONE,
            P1(i8),
            P2(i8),
            BOTH(i8, i8)
        }
        for dir in 0..4{
            let d1 = DIRECTIONS[dir*2];
            let d2 = DIRECTIONS[dir*2 + 1];
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
                State::NONE => { // Next to pos are no symbols -> a new line both active len 1
                    // The two points for the new line
                    println!("None");
                    let p1 = d1 + pos;
                    let p2 = d2 + pos;
                    let mut active = LineActive::BOTH;
                    // If the points are not in the field
                    if !p1.is_valid(){
                        if !p2.is_valid(){
                            continue; // Both points are invalid -> not adding the line
                        }
                        active = LineActive::P2;
                    }
                    else if !p2.is_valid(){
                        active = LineActive::P1;
                    }
                    row.lines.push(Line { p1, p2, active, sign, len: 1 })
                }

                State::P1(i1) => { // Pos is p1 of a line, other field next to it is empty
                    let l1 = row.get(i1);
                    println!("p1");
                    if l1.sign == sign{ // Making one long line
                        if !l1.p1.is_valid(){ // If new point is invalid
                            println!("uhh");
                            match &l1.active {
                                LineActive::P1 => { // Line has no actives anymore -> removing it!
                                    drop(l1);
                                    row.lines.remove(i1 as usize);
                                    continue;
                                } 
                                LineActive::P2 => panic!("Logic error: Pos can't be invalid!"),
                                LineActive::BOTH => l1.active = LineActive::P1,
                            }
                        }
                        l1.len += 1;
                    }
                    else{
                        match &l1.active {
                            LineActive::P1 => { // Line has no actives anymore -> removing it!
                                drop(l1);
                                row.lines.remove(i1 as usize);
                            }
                            LineActive::P2 => panic!("Logic error: Pos can't be invalid!"),
                            LineActive::BOTH => l1.active = LineActive::P1,
                        }
                        if !(pos + d1).is_valid() {
                            continue;
                        }
                        row.lines.push(Line { 
                            p1: pos + d1,
                            p2: pos,
                            active: LineActive::P1,
                            sign: sign,
                            len: 1 
                        });
                    }
                }

                State::P2(i2) => { // Pos is p1 of a line, other field next to it is empty
                    let l2 = row.get(i2);
                    println!("p2");
                    if l2.sign == sign{ // Making one long line
                        l2.p2 = pos + d2;
                        if !l2.p2.is_valid(){ // If new point is invalid
                            println!("uhh");
                            match &l2.active {
                                LineActive::P2 => { // Line has no actives anymore -> removing it!
                                    drop(l2);
                                    row.lines.remove(i2 as usize);
                                    continue;
                                } 
                                LineActive::P1 => panic!("Logic error: Pos can't be invalid!"),
                                LineActive::BOTH => l2.active = LineActive::P2,
                            }
                        }
                        l2.len += 1;
                    }
                    else{
                        match &l2.active {
                            LineActive::P2 => { // Line has no actives anymore -> removing it!
                                drop(l2);
                                row.lines.remove(i2 as usize);
                            }
                            LineActive::P1 => panic!("Logic error: Pos can't be invalid!"),
                            LineActive::BOTH => l2.active = LineActive::P2,
                        }
                        if !(pos + d2).is_valid() {
                            continue;
                        }
                        row.lines.push(Line { 
                            p1: pos,
                            p2: pos + d2,
                            active: LineActive::P2,
                            sign: sign,
                            len: 1
                        });
                    }
                }

                State::BOTH(i1, i2) => {
                    println!("both");
                    if row.get(i1).sign == sign{
                        if row.get(i2).sign == sign{ // Making one long line!
                            let l2 = row.get(i2).clone();
                            let l1 = row.get(i1);

                            l1.len += l2.len + 1;
                            l1.p1 = l2.p1;

                            match &l2.active {
                                LineActive::P2 => {
                                    match &l1.active {
                                        LineActive::P1 => { // Both aren't active anymore -> removing line
                                            drop(l1);
                                            row.lines.remove(max(i1 as usize, i2 as usize));
                                            row.lines.remove(min(i1 as usize, i2 as usize));
                                            continue;
                                        }
                                        LineActive::P2 => panic!("Logic error: Pos can't be invalid!"),
                                        LineActive::BOTH => l1.active = LineActive::P2, // The new p1 is not active anymore -> only P2 active
                                    }
                                }
                                LineActive::P1 => panic!("Logic error: Pos can't be invalid!"),
                                LineActive::BOTH => (), // Nothing changes: l2.p1 is active && l1.p1 was active
                            }
                            row.lines.remove(i2 as usize);
                            continue;
                        }

                        else{ // L1 is same sign, L2 is opposite
                            if row.get(i1).active == LineActive::P1{
                                if row.get(i2).active == LineActive::P2{ // Removing both
                                    row.lines.remove(max(i1 as usize, i2 as usize));
                                    row.lines.remove(min(i1 as usize, i2 as usize));
                                    continue;
                                }
                                else{ // Removing L1
                                    row.get(i2).active = LineActive::P1;

                                    row.lines.remove(i1 as usize);
                                    continue;
                                }
                            }
                            else if row.get(i2).active == LineActive::P2{ // Removing L2
                                row.get(i1).active = LineActive::P2;
                                row.get(i1).len += 1;

                                row.lines.remove(i2 as usize);
                                continue;
                            }
                            else{ // Both remain
                                row.get(i1).active = LineActive::P2;
                                row.get(i1).len += 1;

                                row.get(i2).active = LineActive::P1;
                            }
                        }
                    }

                    else if row.get(i2).sign == sign{ // L2 is same sign. L1 is opposite
                        if row.get(i1).active == LineActive::P1{
                            if row.get(i2).active == LineActive::P2{ // Removing both
                                row.lines.remove(max(i1 as usize, i2 as usize));
                                row.lines.remove(min(i1 as usize, i2 as usize));
                                continue;
                            }
                            else{ // Removing L1
                                row.get(i2).active = LineActive::P1;

                                row.lines.remove(i1 as usize);
                                continue;
                            }
                        }
                        else if row.get(i2).active == LineActive::P2{ // Removing L2
                            row.get(i1).active = LineActive::P2;

                            row.get(i2).len += 1;
                            row.lines.remove(i2 as usize);
                            continue;
                        }
                        else{ // Both remain
                            row.get(i1).active = LineActive::P2;
                            
                            row.get(i2).len += 1;
                            row.get(i2).active = LineActive::P1;
                        }
                    }

                    else{ // Both are opposite sign
                        if row.get(i1).active == LineActive::P1{
                            if row.get(i2).active == LineActive::P2{ // Removing both
                                row.lines.remove(max(i1 as usize, i2 as usize));
                                row.lines.remove(min(i1 as usize, i2 as usize));
                                continue;
                            }
                            else{ // Removing L1
                                row.get(i2).active = LineActive::P1;

                                row.lines.remove(i1 as usize);
                                continue;
                            }
                        }
                        else if row.get(i2).active == LineActive::P2{ // Removing L2
                            row.get(i1).active = LineActive::P2;

                            row.lines.remove(i2 as usize);
                            continue;
                        }
                        else{ // Both remain
                            row.get(i1).active = LineActive::P2;
                            
                            row.get(i2).active = LineActive::P1;
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

#[derive(Clone, PartialEq, Eq)]
enum LineActive{
    P1,
    P2,
    BOTH
}