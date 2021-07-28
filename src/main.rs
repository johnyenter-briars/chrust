#[macro_use]
use std::borrow::BorrowMut;
use std::{io};
mod board;

fn main() {
    let mut brd = board::Board::new(2);
    // brd.set_board();
    brd.load_board();
}
