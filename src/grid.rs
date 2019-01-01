extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Grid {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

// Let [0, 0] be the top left corner

impl Grid {

    fn get_index(&self, x: u32, y: u32) -> usize {
        let index = y * self.width + x;
        return index as usize;
    }



}