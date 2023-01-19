mod line_row;
pub(crate) use line_row::*;

use crate::AlgoPos;

pub(crate) struct Changes{
    
}

pub(crate) struct Lines{
	left_right: [LineRow; 20],
	up_down: [LineRow; 20],
	lu_rd: [LineRow; 40],
	ru_lf: [LineRow; 40],
    values: [[[i16; 2]; 20]; 20]
}

impl Lines {
    pub(crate) fn new() -> Self{
    	let left_right = std::array::from_fn(|_| { LineRow::new() });
    	let up_down = left_right.clone();
    	let lu_rd = Self::create_diag();
    	let ru_lf = Self::create_diag();
    	Lines { left_right, up_down, lu_rd, ru_lf, values: [[[0; 2]; 20]; 20] }
    }

    pub(crate) fn update_move(&mut self, pos: AlgoPos, sign: bool){
    	// Left right
        let update_rows: [*mut LineRow; 4] = [&mut self.left_right[pos.h as usize],
            &mut self.up_down[pos.w as usize],
            &mut self.lu_rd[(pos.w - pos.h + 20) as usize],
            &mut self.ru_lf[(pos.w + pos.h) as usize]];

        unsafe{ LineRow::update_rows(update_rows, pos, sign) }
    }

    pub(crate) fn get_values(&self) -> &[[[i16; 2]; 20]; 20] {
        &&self.values
    }

    pub(crate) fn print(&self){
        println!("Left Right");
        for i in &self.left_right{ i.print() }
        
        println!("Up Down");
        for i in &self.up_down{ i.print() }

        println!("Left-up Right-down");
        for i in &self.lu_rd{ i.print() }

        println!("Right-up Left-down");
        for i in &self.ru_lf{ i.print() }
    }

    fn create_diag() -> [LineRow; 40]{
    	std::array::from_fn(|row|{
    		let score = (row as i32 - 20).abs();
    		return if score == 19{
                LineRow::capacity(0)
            } else if score > 16 {
    			LineRow::capacity(1)
    		}else if score >= 13{
    			LineRow::capacity(2)
    		}else if score >= 11{
    			LineRow::capacity(3)
    		}else{
    			LineRow::new()
    		}
    	})
    }
}

