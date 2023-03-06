mod line_row;

use line_row::*;
use crate::{BoardPos, WIDTH, HEIGHT};

pub(crate) struct Changes{
    old_lines: [(usize, LineRow); 4],
    value_changes: Vec<(usize, i16)>
}

pub(crate) enum UpdateResult{
    Continue(Changes),
    Win(Changes)
}

#[derive(Clone)]
pub(crate) struct Lines{
    lr: [LineRow; WIDTH],
	ud: [LineRow; HEIGHT],
    lu_rd: [LineRow; WIDTH+HEIGHT-1],
    ru_ld: [LineRow; WIDTH+HEIGHT-1],
    values: [i16; WIDTH*HEIGHT]
}

impl Lines {
    pub(crate) fn new() -> Self{

    	let left_right = std::array::from_fn(|_| { LineRow::new() });
    	let up_down = std::array::from_fn(|_| { LineRow::new() });
    	let lu_rd = std::array::from_fn(|_| { LineRow::new() });
    	let ru_ld = std::array::from_fn(|_| { LineRow::new() });

        Lines { lr: left_right, ud: up_down, lu_rd, ru_ld, values: Self::new_values() }
    }

    pub(crate) fn update_move(&mut self, pos: BoardPos, sign: bool) -> UpdateResult{
        let update_rows: [*mut LineRow; 4] = [
            &mut self.lr[pos.h as usize],
            &mut self.ud[pos.w as usize],
            &mut self.lu_rd[(pos.w - pos.h + HEIGHT as i8 - 1) as usize],
            &mut self.ru_ld[(pos.w + pos.h) as usize]];

        let old_lines = [
                (pos.h as usize, self.lr[pos.h as usize].clone()),
                (pos.w as usize, self.ud[pos.w as usize].clone()),
                ((pos.w - pos.h + HEIGHT as i8 - 1) as usize, self.lu_rd[(pos.w - pos.h + HEIGHT as i8 - 1) as usize].clone()),
                ((pos.w + pos.h) as usize, self.ru_ld[(pos.w + pos.h) as usize].clone())
        ];

        if let Some(mut value_changes) = unsafe{ LineRow::update_rows(update_rows, pos, sign) }{
            value_changes.push((pos.get_usize(), -self.values[pos.get_usize()]));
            Self::update_values(&mut self.values, &value_changes);
            UpdateResult::Continue(Changes{ value_changes, old_lines })
        }
        else{
            UpdateResult::Win(Changes { old_lines, value_changes: Vec::new() })
        }
    }

    fn new_values() -> [i16; WIDTH*HEIGHT]{
        let mut values = [0; WIDTH*HEIGHT];
        values[WIDTH/2*HEIGHT+HEIGHT/2] = 1;
        values
    }

    fn update_values(values: &mut [i16; WIDTH*HEIGHT], changes: &Vec<(usize, i16)>){
        for change in changes{
            values[change.0] += change.1;
        }
    }

    pub(crate) fn get_values(&self) -> &[i16; WIDTH*HEIGHT] {
        &self.values
    }

    pub(crate) fn redo_changes(&mut self, changes: Changes){
        self.lr[changes.old_lines[0].0] = changes.old_lines[0].1.clone();
        self.ud[changes.old_lines[1].0] = changes.old_lines[1].1.clone();
        self.lu_rd[changes.old_lines[2].0] = changes.old_lines[2].1.clone();
        self.ru_ld[changes.old_lines[3].0] = changes.old_lines[3].1.clone();

        for i in changes.value_changes{
            self.values[i.0] -= i.1;
        }
    }

    pub(crate) fn print(&self){
        println!("--- LINES ---");
        println!("Left Right");
        for i in &self.lr{ i.print() }
        
        println!("Up Down");
        for i in &self.ud{ i.print() }

        println!("Left-up Right-down");
        for i in &self.lu_rd{ i.print() }

        println!("Right-up Left-down");
        for i in &self.ru_ld{ i.print() }
    }
}
