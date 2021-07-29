extern crate serde_derive;
extern crate serde_json;

// use cell::Cell;
use crate::board::cell::Cell;
use crate::board::cell::piecetype::PieceType;
use core::panic;
use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufReader};

use crate::chessmove::action::{Action, HumanAction};

use crate::board::cell::chesspiece::ChessPiece;
use crate::board::cell::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    squares: Vec<Vec<Cell>>,
    board_size: i32,
}

impl Board {

    //TODO - figure out a way to do this
    // fn get_all_pieces(&self) -> Vec<&Cell> {
    //     self.squares.into_iter().flatten().map(|cell| &cell ).collect()
    // }


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

    pub fn apply_action(&self, action: &dyn Action) {
       self.move_piece(action.get_x(), action.get_y(), action.get_piece_()) 
    }

    pub fn move_piece(&self, x: char, y: i32, piece: &ChessPiece) {
        println!("{:?}", x);
        println!("{:?}", y);
        println!("{:?}", piece);

        // let value = &ChessPiece{
        //     piece_type: PieceType::Knight,
        //     color: Color::Black,
        // };

        // let test = &value;
        // let test2 = &value;


        // println!("{:?}", same);
        for row in  &self.squares {
            for cell in row {
                if let Some(piece_ref) = &cell.space {
                    if std::ptr::eq(piece_ref, piece) {
                        println!("found one!");
                    }
                }
            }
        }

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
