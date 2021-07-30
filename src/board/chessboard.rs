extern crate serde_derive;
extern crate serde_json;

use crate::board::cell::Cell;
use crate::board::cell::piecetype::PieceType;
use crate::chessmove::piecemove::{Move, PieceMove};
use crate::player::humanplayer::HumanPlayer;
use core::panic;
use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufReader};

use crate::chessmove::action::{Action, HumanAction};

use crate::board::cell::chesspiece::ChessPiece;
use crate::board::cell::color::Color;

use super::coordinate::Coordinate;

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
    
    pub fn print_to_screen(&self) {
        // let 
        for row in  &self.squares {
            for cell in row {
                if let Some(piece_ref) = &cell.space {
                    print!("{}", piece_ref.get_str());
                }
            }
        }

    }


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

    pub fn test_move_piece(&mut self, x: char, y: i32, human_player: &HumanPlayer) {

        let target_piece = self.get_piece(x, y);
        
        let valid_piece = self.is_valid_piece(x, y);
         
        if ! valid_piece { return; }

        let first_move = Move{
            x: 'a',
            y: 3,
        };

        let piece_move = PieceMove::new(target_piece, first_move);
        let human_action = HumanAction::new(piece_move, human_player);
        
        let from = Coordinate{
            x: 'a',
            y: 2,
        };

        let to = Coordinate{
            x: 'a',
            y: 3,
        };

        self.move_piece(from, to);       
    }

    pub fn is_valid_piece(&self, x: char, y: i32) -> bool{
        for row in  &self.squares {
            for cell in row {
                if cell.x == x && cell.y == y {
                    if let Some(piece_ref) = &cell.space {
                        return true;
                    }
                    else {
                        return false;
                    }
                }
            }
        }
        panic!("That coordinate pair doesnt exist on the board! x:{},y:{}", x, y);
    }

    pub fn apply_action(&mut self, action: &dyn Action) {
    //    self.move_piece(action.get_x(), action.get_y(), action.get_piece_()) 
    }

    fn set_space_to_empty(&mut self, x: char, y: i32) {
        for row in  self.squares.iter_mut() {
            for cell in row {
                if cell.x == x && cell.y == y {
                    cell.space = Option::from(None);
                }
            }
        }
    }

    pub fn move_piece(&mut self, from: Coordinate, to: Coordinate) {
        println!("{:?}", self.squares);

        //two step process: 
        
        //1: get the reference to the piece we're going to move
        let mut target_piece: ChessPiece = ChessPiece {
            color: Color::Black,
            piece_type: PieceType::Bishop
        };
        for row in  &self.squares {
            for cell in row {
                if cell.x == from.x && cell.y == from.y {
                    if let Some(piece_ref) = cell.space {
                        target_piece = piece_ref; //should be moving the value i hope
                    }
                }
            }
        }

        //2: move the reference to the stop we're moving to 
        for row in  &mut self.squares {
            for cell in row {
                if cell.x == to.x && cell.y == to.y {
                    //we can safely assume there is NO piece at this location
                    //since the from x and y are given to us by the set of possible moves
                    //calculated ahead of time

                    match cell.space {
                        Some(_) => panic!("There should not be a piece already at this location! Something is wrong with the possible move algorythm"),
                        None => {
                            cell.space = Option::from(target_piece);
                        }
                    };
                }
            }
        }

        println!("-------------------------------------");

        // //3: set the current cell to nothing
        self.set_space_to_empty(from.x, from.y);

        println!("{:?}", self.squares);
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
