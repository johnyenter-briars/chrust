extern crate json;

pub mod cell;
use cell::chesspiece::ChessPiece;
use cell::Cell;
use std::borrow::BorrowMut;

use std::char::from_u32_unchecked;
use std::fs::File;
use std::io::{self, BufReader, Read};

#[derive(Debug)]
pub struct Board {
    // squares: [[Cell;8 ]; 8]
    squares: Vec<Vec<Cell>>,
    board_size: i32,
}

impl Board {
    pub fn new(board_size: i32) -> Board {
        let mut squares: Vec<Vec<Cell>> = Vec::new();

        for _ in 0..board_size {
            let mut sub_squares: Vec<Cell> = Vec::new();
            for _ in 0..board_size {
                sub_squares.push(Cell {
                    space: Option::None,
                });
            }

            squares.push(sub_squares);
        }

        Board {
            squares,
            board_size,
        }
    }

    pub fn set_board(&mut self) {
        for i in 0..self.board_size {
            for j in 0..self.board_size {
                let cell: &mut Cell = self.squares[i as usize][j as usize].borrow_mut();

                cell.space = Option::from(ChessPiece::WhitePawn);
            }
        }
    }

    pub fn load_board(&mut self) {
        let mut current_dir = std::env::current_dir().expect("Cant find the path to the current directorey!");

        current_dir.push("src");
        current_dir.push("boards");
        current_dir.push("board1.json");

        let mut file = File::open(current_dir.to_str().expect("cant convert to string :(")).expect("could not open board");

        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .expect("unable to read into the buffer!");

        let doc = json::parse(buffer.as_str()).expect("Parse failed");

        println!("{:?}", doc);
    }
}
