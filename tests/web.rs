//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate rust_wasm_conway;
use rust_wasm_conway::grid::Grid;

wasm_bindgen_test_configure!(run_in_browser);


#[cfg(test)]
pub fn input_spaceship() -> u32 {
    // let mut universe = Grid::new();
    //universe.set_width(6);
    //universe.set_height(6);
    //universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    32
}


#[wasm_bindgen_test]
fn pass() {
    let mut input_universe = input_spaceship();
    assert_eq!(1 + 1, 2);
}
