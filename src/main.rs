use board::Board;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod board;

fn main() {
    let board_result = board::Board::load_from_file("board2");

    let board1: Board = match board_result {
        Ok(brd) => {brd}
        Err(error)  => panic!("error! {}", error)
    };
    println!("{:?} ", board1);
}

