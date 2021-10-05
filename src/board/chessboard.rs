extern crate serde_derive;
extern crate serde_json;

use crate::board::cell::Cell;
use crate::board::cell::piecetype::PieceType;
use crate::chessmove::piecemove::{PieceMove};
use crate::player::humanplayer::HumanPlayer;
use std::error::Error;
use core::result::Result;

use crate::board::cell::chesspiece::ChessPiece;
use crate::board::cell::color::Color;

use super::coordinate::Coordinate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub squares: Vec<Vec<Cell>>,
    pub board_size: i32,
}

impl Board {
    pub fn test_cell_at(&self, coordinate: Coordinate) -> Option<&Cell> {
        self.all_cells().iter().filter(|cell| cell.x == coordinate.x && cell.y == coordinate.y).map(|x| x.to_owned()).next()
    }

    //should really ONLY be used in an evaluation function
    pub fn all_cells(&self) -> Vec<&Cell> {
        self.squares.iter().flatten().collect()
    }

    fn all_cells_mut(&mut self) -> Vec<&mut Cell> {
        self.squares.iter_mut().flatten().collect()
    }

    pub fn print_to_screen(&self, configuration_name: String) {
        println!("---------------------------{}", configuration_name);
        for row in  &self.squares {
            if let Some(first_cell) = row.into_iter().next() {
                print!("{}|", first_cell.y);
            }
            for cell in row {
                if let Some(piece_ref) = &cell.space {
                    print!(" {} |", piece_ref.str());
                } else {
                    print!("   |");
                }
            }
            println!("");
        }
        print!(" ");
        "abcdefgh".to_string().chars().into_iter().for_each(|int| print!("  {} ", int));
        println!("");
    }

    pub fn piece(&self, x: char, y: i32) -> core::result::Result<&ChessPiece, Box<dyn Error>> {
        for cell in self.all_cells() {
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

    pub fn possible_moves_human(&self, current_position: Coordinate, human_player: &HumanPlayer) -> Result<Vec<Coordinate>, Box<dyn Error>> {
        let target_piece = self.piece(current_position.x, current_position.y)?;

        if target_piece.color != human_player.color {
            return Err(Box::from("Thats not your piece!"));
        }
         
        Ok(target_piece.piece_type.available_moves(target_piece, current_position, self))
    }

    pub fn possible_moves(&self, current_position: Coordinate, color: Color) -> Result<Vec<Coordinate>, Box<dyn Error>> {
        let target_piece = self.piece(current_position.x, current_position.y)?;

        if target_piece.color != color {
            return Err(Box::from("Thats not your piece!"));
        }
         
        Ok(target_piece.piece_type.available_moves(target_piece, current_position, self))
    }

    pub fn all_possible_moves(&self, color: Color) -> Result<Vec<PieceMove>, Box<dyn Error>> {
        let cells = self.cells_with_pieces_with_color(color);

        let mut possible_moves: Vec<PieceMove> = Vec::new();

        for cell in cells{
            let current_position = Coordinate::new(cell.x, cell.y);
            let piece_at_position = self.piece(current_position.x, current_position.y)?;

            let coord_choices = self.possible_moves(current_position, color)?;

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

    fn space_to_empty(&mut self, x: char, y: i32) {
        for row in  self.squares.iter_mut() {
            for cell in row {
                if cell.x == x && cell.y == y {
                    cell.space = Option::from(None);
                }
            }
        }
    }

    pub fn pieces_of_color(&self, color: Color) -> Vec<&ChessPiece> {
        let mut pieces: Vec<&ChessPiece> = Vec::new();
        for cell in self.all_cells() {
            if let Some(piece_ref) = &cell.space {
                if piece_ref.color == color {
                    pieces.push(piece_ref);
                }
            }
        }
        pieces
    }

    pub fn cells_with_pieces_with_color(&self, color: Color) -> Vec<&Cell> {
        let mut cells: Vec<&Cell> = Vec::new();
        for cell in self.all_cells() {
            if let Some(piece_ref) = &cell.space {
                if piece_ref.color == color {
                    cells.push(cell);
                }
            }
        }
        cells
    }

    pub fn piece_specific(&self, color: Color, piece_type: PieceType) -> Option<&ChessPiece>{
        let pieces = self.pieces_of_color(color);

        let piece = pieces.into_iter().find(|x| x.piece_type == piece_type);
        
        piece
    }


    pub fn move_piece(&mut self, from: Coordinate, to: Coordinate){
        //1: get the reference to the piece we're going to movmute
        let mut target_piece: ChessPiece = ChessPiece {
            color: Color::Black,
            piece_type: PieceType::Bishop
        };
        for cell in self.all_cells(){
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
                        Some(_) => {
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

        //3: set the current cell to nothing
        self.space_to_empty(from.x, from.y);
    }

    pub fn load_from_file(board_name: &str) -> Result<Board, Box<dyn std::error::Error>> {
        let board_string = match board_name {
            "game_start" => include_str!("../../boards/game_start.json"),
            "empty_board" => include_str!("../../boards/empty_board.json"),
            _ => {return Err(Box::from("Invalid option for board_name"))}
        };
        
        let board: Board = serde_json::from_str(board_string)?;

        if board.squares.len() as i32 != board.board_size {
            return Err(Box::from(format!(
                "The json board is formated incorectly! The expected board_size: {} does not match the actual board size: {}",
                board.board_size, 
                board.squares.len()
            )));
        }

        Ok(board)
    }
    
    // assumes the game section of the fen has been taken out
    pub fn load_from_fen(fen: String) -> Result<Board, Box<dyn std::error::Error>> {
        let mut board = Board::load_from_file("empty_board")?;
        let sections: Vec<Vec<char>> = fen.split('/').into_iter().map(|s| s.to_string().chars().collect()).collect();

        let mut cells = board.all_cells_mut();
        let mut cell_iter = cells.iter_mut();

        for c_vec in sections {
            for c in c_vec {
                if c.is_numeric() {
                    let num_to_skip = c.to_digit(10).expect("Unable to convert the num to skip to i32");
                    for _ in 0..num_to_skip {
                        cell_iter.next();
                    }
                }
                else {
                    let cell = cell_iter.next().expect("The iterator really shouldnt be empty :(");
                    cell.space.get_or_insert(piece_from_fen_char(c)?);
                }
            }
        }

        Ok(board)
    }

    pub fn board_fen_section(&self) -> String {
        let mut fen = "".to_string();

        //construct the piece position portion
        for row in &self.squares {
            let mut num_empty_squares:i32 = 0;
            for cell in row {
                if let Some(piece) = cell.space {
                    if num_empty_squares == 0 {
                        fen.push_str(piece.fen_code());
                    } else {
                        fen.push_str(&num_empty_squares.to_string());
                        fen.push_str(piece.fen_code());
                    }
                    num_empty_squares = 0;
                } else {
                    num_empty_squares += 1;
                }
            }
            if num_empty_squares != 0 {
                fen.push_str(&num_empty_squares.to_string());
            }
            fen.push_str("/");
        }

        fen[0..fen.len()-1].to_string()
    }
}

fn piece_from_fen_char(c: char) -> Result<ChessPiece, Box<dyn std::error::Error>> {
    match c {
        'p' => Ok(ChessPiece{piece_type: PieceType::Pawn, color: Color::Black}),
        'r' => Ok(ChessPiece{piece_type: PieceType::Rook, color: Color::Black}),
        'n' => Ok(ChessPiece{piece_type: PieceType::Knight, color: Color::Black}),
        'b' => Ok(ChessPiece{piece_type: PieceType::Bishop, color: Color::Black}),
        'k' => Ok(ChessPiece{piece_type: PieceType::King, color: Color::Black}),
        'q' => Ok(ChessPiece{piece_type: PieceType::Queen, color: Color::Black}),
        'P' => Ok(ChessPiece{piece_type: PieceType::Pawn, color: Color::White}),
        'R' => Ok(ChessPiece{piece_type: PieceType::Rook, color: Color::White}),
        'N' => Ok(ChessPiece{piece_type: PieceType::Knight, color: Color::White}),
        'B' => Ok(ChessPiece{piece_type: PieceType::Bishop, color: Color::White}),
        'K' => Ok(ChessPiece{piece_type: PieceType::King, color: Color::White}),
        'Q' => Ok(ChessPiece{piece_type: PieceType::Queen, color: Color::White}),
        _ => Err(Box::from("Invalid char: {}")),
    }
}
