extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::fmt;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[derive(Debug, Clone)]
pub struct Grid {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

// Let [0, 0] be the top left corner

impl Grid {

    fn new() {
        
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        let index = y * self.width + x;
        return index as usize;
    }

}

impl fmt::Display for Grid {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.cells[..self.width as usize] {
            for &cell in line {
                if cell == Cell::Dead write!(f, "{}", "o") else write!(f, "{}", "1")?;                
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}





/*
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
*/