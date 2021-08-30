extern crate serde_derive;
extern crate serde_json;

use crate::board::cell::Cell;
use crate::board::cell::piecetype::PieceType;
use crate::chessmove::piecemove::{PieceMove};
use crate::player::aiplayer::AIPlayer;
use crate::player::humanplayer::HumanPlayer;
use core::panic;
use std::borrow::Borrow;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
use core::result::Result;

use crate::chessmove::action::{Action, HumanAction};

use crate::board::cell::chesspiece::ChessPiece;
use crate::board::cell::color::Color;

use super::coordinate::Coordinate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub squares: Vec<Vec<Cell>>,
    pub board_size: i32,
}

impl Board {

    // TODO - figure out a way to do this
    // fn get_all_pieces(&self) -> Vec<&Cell> {
    //     self.squares.into_iter().flatten().map(|cell| &cell ).collect()
    // }

    pub fn get_cell_at(&self, coordinate: Coordinate) -> &Cell {
        match self.get_all_cells().iter().filter(|cell| cell.x == coordinate.x && cell.y == coordinate.y).next() {
            Some(cell) => cell,
            None => panic!("No cell found at: {:?}", coordinate),
        }
    }

    pub fn test_cell_at(&self, coordinate: Coordinate) -> Option<&Cell> {
        self.get_all_cells().iter().filter(|cell| cell.x == coordinate.x && cell.y == coordinate.y).map(|x| x.to_owned()).next()
    }

    fn get_all_cells(&self) -> Vec<&Cell> {
        self.squares.iter().flatten().collect()
    }

    //should really ONLY be used in an evaluation function
    pub fn get_all_pieces<'a>(&self) -> Vec<ChessPiece>{
        let im_worried_this_will_move: Vec<ChessPiece> = self.get_all_cells().iter().map(|cell| cell.space).into_iter().flat_map(|e| e).collect();
        im_worried_this_will_move
    }

    pub fn get_empty_spaces(&self) -> Vec<Coordinate>{
        self.get_all_cells().iter().filter(|cell| cell.is_empty()).map(|cell| Coordinate::new(cell.x, cell.y)).collect()
    }
    
    pub fn print_to_screen(&self, configuration_name: String) {
        println!("-----------------------------{}", configuration_name);
        for row in  &self.squares {
            if let Some(first_cell) = row.into_iter().next() {
                print!("{}", first_cell.y);
            }
            for cell in row {
                if let Some(piece_ref) = &cell.space {
                    print!(" {} ", piece_ref.get_str());
                } else {
                    print!("   ");
                }
            }
            println!("");
            println!("");
        }
        print!(" ");
        "abcdefgh".to_string().chars().into_iter().for_each(|int| print!(" {} ", int));
        println!("");
    }

    pub fn get_piece(&self, x: char, y: i32) -> core::result::Result<&ChessPiece, Box<dyn Error>> {
        for cell in self.get_all_cells() {
            if cell.x == x && cell.y == y {
                if let Some(piece_ref) = &cell.space {
                    return Ok(piece_ref);
                }
                else {
                    return Err(Box::from(format!("You tried to get a piece at a space that didnt have one - did you mean to do that? x:{},y:{}", x, y)));
                }
            }
        }
        
        Err(Box::from(format!("couldnt find the given pairs of points on the board! Either the board is goofed - or your points are: x:{},y:{}", x, y)))
    }

    pub fn get_possible_moves_human(&self, current_position: Coordinate, turn_num: i32, human_player: &HumanPlayer) -> Result<Vec<Coordinate>, Box<dyn Error>> {
        let target_piece = self.get_piece(current_position.x, current_position.y)?;

        if target_piece.color != human_player.color {
            return Err(Box::from("Thats not your piece!"));
        }
         
        Ok(target_piece.piece_type.available_moves(target_piece, current_position, self, turn_num))
    }

    pub fn get_possible_moves(&self, current_position: Coordinate, turn_num: i32, color: Color) -> Result<Vec<Coordinate>, Box<dyn Error>> {
        let target_piece = self.get_piece(current_position.x, current_position.y)?;

        if target_piece.color != color {
            return Err(Box::from("Thats not your piece!"));
        }
         
        Ok(target_piece.piece_type.available_moves(target_piece, current_position, self, turn_num))
    }

    pub fn get_all_possible_moves(&self, turn_num: i32, color: Color) -> Result<Vec<PieceMove>, Box<dyn Error>> {
        let cells = self.get_cells_with_pieces_with_color(color);

        let mut possible_moves: Vec<PieceMove> = Vec::new();

        for cell in cells{
            let current_position = Coordinate::new(cell.x, cell.y);
            let piece_at_position = self.get_piece(current_position.x, current_position.y)?;
            let x = match piece_at_position.piece_type {
                PieceType::Queen => 10,
                _ => 5
            };

            let coord_choices = self.get_possible_moves(current_position, turn_num, color)?;

            let mut possible_moves_for_piece: Vec<PieceMove> = 
                coord_choices.iter().map(|coord | PieceMove::new(&piece_at_position, current_position, coord.clone())).collect();

            
            possible_moves.append(&mut possible_moves_for_piece);
        }

        Ok(possible_moves)
    }


    pub fn apply_action(&self, action: &PieceMove) -> Board {
        //by jove i hope this doesnt have unexpected side affects
        let mut new_board = self.clone();
        new_board.move_piece(action.from, action.to);
        new_board
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

    pub fn get_pieces_of_color(&self, color: Color) -> Vec<&ChessPiece> {
        let mut pieces: Vec<&ChessPiece> = Vec::new();
        for cell in self.get_all_cells() {
            if let Some(piece_ref) = &cell.space {
                if piece_ref.color == color {
                    pieces.push(piece_ref);
                }
            }
        }
        pieces
    }

    pub fn get_cells_with_pieces_with_color(&self, color: Color) -> Vec<&Cell> {
        let mut cells: Vec<&Cell> = Vec::new();
        for cell in self.get_all_cells() {
            if let Some(piece_ref) = &cell.space {
                if piece_ref.color == color {
                    cells.push(cell);
                }
            }
        }
        cells
    }

    pub fn get_piece_specific(&self, color: Color, piece_type: PieceType) -> Option<&ChessPiece>{
        let pieces = self.get_pieces_of_color(color);

        let piece = pieces.into_iter().find(|x| x.piece_type == piece_type);
        
        piece
    }


    pub fn move_piece(&mut self, from: Coordinate, to: Coordinate) {
        //1: get the reference to the piece we're going to move
        let mut target_piece: ChessPiece = ChessPiece {
            color: Color::Black,
            piece_type: PieceType::Bishop
        };
        for cell in self.get_all_cells(){
            if cell.x == from.x && cell.y == from.y {
                if let Some(piece_ref) = cell.space {
                    target_piece = piece_ref; //should be moving the value i hope
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
                        Some(captured_piece) => {
                            //TODO - handle the piece being captured
                            cell.space = Option::from(target_piece);
                        }
                        None => {
                            cell.space = Option::from(target_piece);
                        }
                    };
                }
            }
        }

        // //3: set the current cell to nothing
        self.set_space_to_empty(from.x, from.y);

    }

    pub fn load_from_file(board_name: &str) -> Result<Board, Box<dyn std::error::Error>> {
        let mut current_dir =
            std::env::current_dir().expect("Cant find the path to the current directory!");

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
