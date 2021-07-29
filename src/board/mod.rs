extern crate serde_derive;
extern crate serde_json;

pub mod cell;
use cell::Cell;
use core::panic;
use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufReader};

use self::cell::chesspiece::ChessPiece;
use self::cell::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    squares: Vec<Vec<Cell>>,
    board_size: i32,
}

impl Board {
    pub fn get_piece(&self, x: char, y: i32) -> &ChessPiece {
        for row in  &self.squares {
            for cell in row {
                if cell.x == x && cell.y == y {
                    if let Some(piece_ref) = &cell.space {
                        return piece_ref;
                    }
                    else {
                        panic!("You tried to get a piece at a space that didnt have one - did you mean to do that? x:{},y:{}", x, y);
                    }
                }
            }
        }

        panic!("couldnt find the given pairs of points on the board! Either the board is goofed - or your points are: x:{},y:{}", x,y)
    }

    pub fn load_from_file(board_name: &str) -> Result<Board, Box<dyn std::error::Error>> {
        let mut current_dir =
            std::env::current_dir().expect("Cant find the path to the current directory!");

        current_dir.push("src");
        current_dir.push("boards");
        current_dir.push(format!("{}.json", board_name));

        if ! current_dir.is_file() {
            return Err(Box::from("Not a valid file path to board!"));
        }

        let file = File::open(current_dir)?;

        let reader = BufReader::new(file);

        let board: Board = serde_json::from_reader(reader)?;

        if board.squares.len() as i32 != board.board_size {
            return Err(Box::from(format!(
                "The json board is formated incorectly! The expected board_size: {} does not match the actual board size: {}",
                board.board_size, 
                board.squares.len()
            )));
        }

        Ok(board)
    }
}
