use std::fmt;
use std::io;

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate rand;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

// Doesn't work, can't find macro log! in this scope
// #[macro_use]
// use crate::utils::*;

// TODO: Put into the util module.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CountFn {
    Count_01,
    Count_02
}

/*
pub struct Cellulle {
    state: CellState
} */

#[wasm_bindgen]
#[derive(Clone)]            // Removed the Debug trait, as it's not implemented for the fn, so the whole struct can't be cast as Debug.
pub struct Grid {
    pub width: u32,
    pub height: u32,
    cells: Vec<CellState>,       // WASM: When it's pub, error: trait is not implemented for `std::boxed::Box<[grid::CellState]>`
    tick_fn: fn(&mut Grid, Vec<u8>)
}

// impl CellState {
//   
//    pub fn flip() -> CellState {    // OPTIM: Could be through a bitwise operation.
//    }
// }

#[wasm_bindgen]
impl Grid {
    
    pub fn new() -> Grid {
        Grid::new_custom(128, 128)
    }

    pub fn new_custom(w: u32, h: u32) -> Grid {
        let new_cells: Vec<CellState> = (0..w*h)     
            .map(|i|

                 if i % 2 == 0 || i % 9 == 0 { CellState::Alive } else { CellState::Dead }

                )
            .collect();     // Q: vs. &self.cells?

        // log!(
        //  "DBG/ new(): new_cells: {:?}",
        //   new_cells.clone()
        // );
        
        Grid {
            width: w,       // Q: vs. just width, defined by let in the new() scope? Rust shortcut?
            height: h,      // A: Yep. More in "Destructuring Structs" manual section.
            cells: new_cells,
            tick_fn: Grid::tick_neighbor_matrix_0 // Q: This should be the "normal" ruleset, no? Weird tho! Mixed w/ high-life?
            
        }
    }

    pub fn tick(&mut self) {
        let mut nm = self.compute_neighbor_matrix_0();
        (self.tick_fn)(self, nm);
    }
    
    pub fn render(&self) -> String {

        // log!("Rendered: {:?}", (*self).cells());
        self.to_string()             // Q: Does it use the fmt::Display function? Or?
    }

    /*
    pub fn get_index(&self, x: u32, y: u32) -> usize {
        
        let (x, y) = (self.wrap_x(x), self.wrap_y(y));
    
        (y * self.width + x) as usize
    } */

    pub fn get_index(&self, x: u32, y: u32) -> usize {
        
        // let (x, y) = (self.wrap_x(x), self.wrap_y(y));
    
        (y * self.width + x) as usize
    }

    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn cells(&self) -> *const CellState {
        return self.cells.as_ptr();
    }

    // SETTERS

    // OPTIM: Include the wrapping in the ge
    pub fn set_alive(&mut self, x: u32, y: u32) {
        let index = self.get_index(x, y);
        self.cells[index] = CellState::Alive;
    }

    // TMP: Malfunct wrapping so far. Just a 1D array insert, ignoring the 2D structure.
    /// @arguments
    ///     pattern: Vec<u8> - should be of size 9. Better an array? Or? Where/how ideally checked?
    pub fn add_pattern(&mut self, x: u32, y: u32, pattern: Vec<u8>) {      // OPTIM: Use a an array or a bitmask.

        for i_y in 0..=2 {
            for i_x in 0..=2 {
                let idx = self.get_index(x + i_x, y + i_y);         // Q: When I put the self.blabla directly as an index, I get the immutable borrow error. Why?
                let idx_pattern = (i_y * 3 + i_x) as usize;

                self.cells[idx] = match pattern[idx_pattern] {
                    0 => CellState::Dead,
                    1 => CellState::Alive,
                    _ => CellState::Dead
                }
            }
        }

        // TODO Error handling: array overflows etc.
    }

    pub fn get_neighbors(&self, x: u32, y: u32) -> Vec<u8> {
        return vec![0, 1, 0, 1, 0, 1, 0, 1];
    }

    pub fn toggle(&mut self, x: u32, y: u32) -> CellState {

        let idx = self.get_index(x, y);

        self.cells[idx] = match self.cells[idx] {   // OPTIM: Use a bitwise operator.
            CellState::Alive => CellState::Dead,
            CellState::Dead => CellState::Alive
        };
        
        return self.cells[idx];
    }

    pub fn count_alive_neighbors(&self, x: u32, y: u32) -> u8 {

        let mut count = 0;

        for n in 0..=2 {
            for m in 0..=2 {
                if m == 1 && n == 1 { continue; } // We don't count current cell as its own neighbor.
                // LEARNING: Tady byla chyba!! Má být obojí 1 (protože odečítám 1), a já měl m, n == 0!
                
                // Wrap                
                // !!! The wrap-around doesn't seem to be working.
                let xw = self.wrap_x(x + m - 1);
                let yw = self.wrap_y(y + n - 1);
                // let xw = x;
                // let yw = y;
                

                // log!("X: {}, Y: {}, XW: {}, YW: {}", x, y, xw, yw);

                count += match self.cells[self.get_index(xw, yw)] {    
                    CellState::Alive => 1,
                    CellState::Dead => 0
                };


            }      
        }  
        // log!("{}", count);
        return count;        
    }
    

    pub fn compute_neighbor_matrix_0 (&self) -> Vec<u8> {

        let mut neighbor_matrix: Vec<u8> = vec![0; self.width as usize * self.height as usize];

        // TODO: Ignoring the borders for now.
        // Learning: THIS should be the first implementation to get done.
        /// @ Q: Complexity: quadratic?

        /// TODO: Using widened boundaries right now.
        /// Q: Starting from 0 vs. from 1 - huge difference in the resulting lifetime of the sim. How come?
        for y in 0..=self.height-1 {
        // OPTIM: How costly is checking the "if" / "match" condition vs. some kind of mul / mod?
            for x in 0..=self.width-1 {             
                neighbor_matrix[self.get_index(x, y)] = self.count_alive_neighbors(x, y); // (x * y % 8) as u8; // rand::random::<u8>() % 8;// self.count_alive_neighbors(x, y);
            }
        }    
        // log!("{:?}", neighbor_matrix.clone());        
        return neighbor_matrix;
    }

    pub fn compute_neighbor_matrix_2 (&self) -> Vec<u8> {

        let mut neighbor_matrix: Vec<u8> = vec![0; self.width as usize * self.height as usize];

        // TODO: Ignoring the borders for now.
        // Learning: THIS should be the first implementation to get done.
        /// @ Q: Complexity: quadratic?

        for i in 1..=self.height-2 {
        // OPTIM: How costly is checking the "if" / "match" condition vs. some kind of mul / mod?
            for j in 1..=self.width-2 {             
                neighbor_matrix[self.get_index(j, i)] = self.count_alive_neighbors(j, i);
            }
        }    
        return neighbor_matrix;
    }    

    pub fn compute_neighbor_matrix_1 (&self) -> Vec<u8> {
        // let mut neighbor_matrix: Vec<u8> = Vec::with_capacity(self.width as usize * self.height as usize);
        let mut neighbor_matrix = vec![0; self.width as usize * self.height as usize];

        // TODO: Ignoring the borders for now.
        // Learning: THIS should be the first implementation to get done.
        /// @ Q: Complexity: quadratic?
        for i in 1..=self.height-2 {
            for j in 1..=self.width-2 {
                let mut count = 0;
                for n in 0..=2 {
                    for m in 0..=2 {
                        if m == 0 && n == 0
                            { continue; } // We don't count current cell as its own neighbor.
                        count += self.cells[self.get_index(i+n-1, j+m-1) as usize] as u8;
                    }
                }                
                neighbor_matrix[self.get_index(i, j) as usize] = count;
            }
        }
        
        return neighbor_matrix as Vec<u8>;
    }   


    fn tick_neighbor_matrix_0(&mut self, alive_neighbor_matrix: Vec<u8>) {
        let a: () = self.cells.iter_mut().enumerate()                 // Type () as we don't actually map and use the collected items, we just change the *cell.
            .map(|(i, cell)| { *cell = match (*cell, alive_neighbor_matrix[i as usize]) {   
                    (CellState::Alive, 0..=1) => CellState::Dead,
                    (CellState::Dead, 3) => CellState::Alive,
                    (CellState::Alive, 4..=8) => CellState::Dead,
                    (otherwise, _) => otherwise
            }           
            }).collect(); // Map is zazy, doesn't do anything until consumed.      
    }

    fn tick_neighbor_matrix_1(&mut self, alive_neighbor_matrix: Vec<u8>) {        

        for (i, cell) in self.cells.iter_mut().enumerate() {
            *cell = match (*cell, alive_neighbor_matrix[i as usize]) {
                    (CellState::Alive, 0..=1) => CellState::Dead,                    
                    (CellState::Dead, 3) => CellState::Alive,
                    (CellState::Alive, 4..=8) => CellState::Dead,
                    (otherwise, _) => otherwise
            }
        };
    }

    fn tick_neighbor_matrix_high_life(&mut self, alive_neighbor_matrix: Vec<u8>) {        
     
        for (i, cell) in self.cells.iter_mut().enumerate() {
            *cell = match (*cell, alive_neighbor_matrix[i as usize]) {
                    (CellState::Alive, 0..=1) => CellState::Dead,                    
                    (CellState::Dead, 3) | (CellState::Dead, 6) => CellState::Alive,
                    (CellState::Alive, 4..=8) => CellState::Dead,
                    (otherwise, _) => otherwise
            }
        };
    }

    // DUMMY
    // pub fn count_alive_neighbors_1(&self, x:u32, y: u32) -> u8 {        // WASM: Returning and taking tuples as parameters not yet working in wasm-bindgen.
//
  //         8
    //}
    
    // Q: Could be optimized, as a closure perhaps.
    fn wrap_x(&self, x: u32) -> u32 {
        Grid::wrap(x, self.width)
    }

    fn wrap_y(&self, y: u32) -> u32 {
        Grid::wrap(y, self.height)
    }
    
    fn wrap(i: u32, range: u32) -> u32 {
        // ((i % (range)) + (range) * (i == 0) as u32) // OPTIM: Optimize "j == 0" with some bitwise operator? Vs. just adding/substracting range?
        let a = match i {
            i if i > range-1 => 0,
            i if i < 0 => range-1,
            otherwise => i
        };

        // log!("{}", a);
        a

    }
}

impl fmt::Display for Grid {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut j: u32 = 0;

        // Optim: This is the same iterative function as used elsewhere. Could generalize.
        for (i, line) in self.cells.as_slice().chunks(self.width as usize).enumerate() {
            for &cell in line {
                let symbol = if cell == CellState::Dead { '◻' } else { '◼' };       // Q: i as u8 as char - isn't this dangerous? What if i big? Test.
                write!(f, "{:} ", symbol); 
                j += 1;
            }
            j = 0;
            write!(f, "\n")?;
        }
        Ok(())
    }
}