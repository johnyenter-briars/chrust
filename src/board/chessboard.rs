extern crate serde_derive;
extern crate serde_json;

use crate::board::piece::pawn::Pawn;
use crate::chessmove::piecemove::PieceMove;
// use crate::board::cell::piecetype::PieceType;
// use crate::chessmove::piecemove::{PieceMove};
use crate::player::humanplayer::HumanPlayer;
use core::result::Result;
use std::borrow::BorrowMut;
use std::error::Error;
use std::vec;

// use crate::board::cell::chesspiece::ChessPiece;

use super::cell::Cell;
use super::color::Color;
use super::coordinate::Coordinate;
use super::piece::achesspiece::AChessPiece;
use super::piece::bishop::Bishop;
use super::piece::king::King;
use super::piece::knight::Knight;
use super::piece::queen::Queen;
use super::piece::rook::Rook;
use super::piecetype::PieceType;
// use super::piecetype::PieceType;

#[derive(Debug)]
pub struct ChessBoard {
    pub squares: Vec<Vec<Cell>>,
    pub board_size: i32,
}

impl ChessBoard {
    pub fn test_cell_at(&self, coordinate: Coordinate) -> Option<&Cell> {
        self.all_cells()
            .iter()
            .filter(|cell| cell.x == coordinate.x && cell.y == coordinate.y)
            .map(|x| x.to_owned())
            .next()
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
        for row in &self.squares {
            if let Some(first_cell) = row.iter().next() {
                print!("{}|", first_cell.y);
            }
            for cell in row {
                if let Some(piece_ref) = &cell.space {
                    print!(" {} |", piece_ref.str());
                } else {
                    print!("   |");
                }
            }
            println!();
        }
        print!(" ");
        "abcdefgh"
            .to_string()
            .chars()
            .for_each(|int| print!("  {} ", int));
        println!();
    }

    // Returns the chess piece at a given position
    pub fn piece(
        &self,
        x: char,
        y: i32,
    ) -> core::result::Result<&Box<dyn AChessPiece>, Box<dyn Error>> {
        for cell in self.all_cells() {
            if cell.x == x && cell.y == y {
                if let Some(piece_ref) = &cell.space {
                    return Ok(piece_ref);
                } else {
                    return Err(Box::from(format!("You tried to get a piece at a space that didnt have one - did you mean to do that? x:{},y:{}", x, y)));
                }
            }
        }

        Err(Box::from(format!("couldnt find the given pairs of points on the board! Either the board is goofed - or your points are: x:{},y:{}", x, y)))
    }

    pub fn possible_moves_human(
        &self,
        current_position: Coordinate,
        human_player: &HumanPlayer,
    ) -> Result<Vec<Coordinate>, Box<dyn Error>> {
        let target_piece = self.piece(current_position.x, current_position.y)?;

        if target_piece.color() != human_player.color {
            return Err(Box::from("Thats not your piece!"));
        }

        Ok(target_piece.available_moves(self))
    }

    pub fn possible_moves(
        &self,
        current_position: Coordinate,
        color: Color,
    ) -> Result<Vec<Coordinate>, Box<dyn Error>> {
        let target_piece = self.piece(current_position.x, current_position.y)?;

        if target_piece.color() != color {
            return Err(Box::from("Thats not your piece!"));
        }

        Ok(target_piece.available_moves(self))
    }

    pub fn all_possible_moves(&self, color: Color) -> Result<Vec<PieceMove>, Box<dyn Error>> {
        let cells = self.cells_with_pieces_with_color(color);

        let mut possible_moves: Vec<PieceMove> = Vec::new();

        for cell in cells {
            let current_position = Coordinate::new(cell.x, cell.y);
            let piece_at_position = self.piece(current_position.x, current_position.y)?;

            let coord_choices = self.possible_moves(current_position, color)?;

            todo!()
            // let mut possible_moves_for_piece: Vec<PieceMove> =
            //     coord_choices.iter().map(|coord | PieceMove::new(piece_at_position, current_position, *coord)).collect();

            // possible_moves.append(&mut possible_moves_for_piece);
        }

        Ok(possible_moves)
    }

    pub fn apply_action(&self, action: &PieceMove) -> ChessBoard {
        //by jove i hope this doesnt have unexpected side affects
        // let new_board: &mut Board = self.clone();
        // new_board.move_piece(action.from, action.to);
        // *new_board
        todo!("just create a new board object")
    }

    fn space_to_empty(&mut self, x: char, y: i32) {
        for row in self.squares.iter_mut() {
            for cell in row {
                if cell.x == x && cell.y == y {
                    cell.space = None;
                }
            }
        }
    }

    pub fn pieces_of_color(&self, color: Color) -> Vec<Box<dyn AChessPiece>> {
        // let mut pieces: Vec<&ChessPiece> = Vec::new();
        // for cell in self.all_cells() {
        //     if let Some(piece_ref) = &cell.space {
        //         if piece_ref.color == color {
        //             pieces.push(piece_ref);
        //         }
        //     }
        // }
        // pieces
        todo!()
    }

    pub fn cells_with_pieces_with_color(&self, color: Color) -> Vec<&Cell> {
        let mut cells: Vec<&Cell> = Vec::new();
        for cell in self.all_cells() {
            if let Some(piece_ref) = &cell.space {
                if piece_ref.color() == color {
                    cells.push(cell);
                }
            }
        }
        cells
    }

    pub fn piece_specific(
        &self,
        color: Color,
        piece_type: PieceType,
    ) -> Option<Box<&dyn AChessPiece>> {
        let pieces = self.pieces_of_color(color);

        // pieces.into_iter().find(|x| x.piece_type == piece_type)
        todo!()
    }

    pub fn move_piece(&mut self, from: Coordinate, to: Coordinate) {
        //1: get the reference to the piece we're going to move
        todo!();
        // let mut target_piece: ChessPiece = ChessPiece {
        //     color: Color::Black,
        //     current_position: Coordinate { x: 'a', y: 10 }
        // };
        // for cell in self.all_cells(){
        //     if cell.x == from.x && cell.y == from.y {
        //         if let Some(piece_ref) = cell.space {
        //             target_piece = piece_ref; //should be moving the value i hope
        //         }
        //     }
        // }

        // //2: move the reference to the stop we're moving to
        // for row in  &mut self.squares {
        //     for cell in row {
        //         if cell.x == to.x && cell.y == to.y {
        //             //we can safely assume there is NO piece at this location
        //             //since the from x and y are given to us by the set of possible moves
        //             //calculated ahead of time

        //             match cell.space {
        //                 Some(_) => {
        //                     //TODO - handle the piece being captured
        //                     cell.space = Option::from(target_piece);
        //                 }
        //                 None => {
        //                     cell.space = Option::from(target_piece);
        //                 }
        //             };
        //         }
        //     }
        // }

        //3: set the current cell to nothing
        self.space_to_empty(from.x, from.y);
    }

    pub fn load_empty_board() -> Result<ChessBoard, Box<dyn std::error::Error>> {
        let board = ChessBoard {
            squares: vec![
                vec![
                    Cell::new('a', 8, Color::Black, None),
                    Cell::new('b', 8, Color::Black, None),
                    Cell::new('c', 8, Color::Black, None),
                    Cell::new('d', 8, Color::Black, None),
                    Cell::new('e', 8, Color::Black, None),
                    Cell::new('f', 8, Color::Black, None),
                    Cell::new('g', 8, Color::Black, None),
                    Cell::new('h', 8, Color::Black, None),
                ],
                vec![
                    Cell::new('a', 7, Color::Black, None),
                    Cell::new('b', 7, Color::Black, None),
                    Cell::new('c', 7, Color::Black, None),
                    Cell::new('d', 7, Color::Black, None),
                    Cell::new('e', 7, Color::Black, None),
                    Cell::new('f', 7, Color::Black, None),
                    Cell::new('g', 7, Color::Black, None),
                    Cell::new('h', 7, Color::Black, None),
                ],
                vec![
                    Cell::new('a', 6, Color::Black, None),
                    Cell::new('b', 6, Color::Black, None),
                    Cell::new('c', 6, Color::Black, None),
                    Cell::new('d', 6, Color::Black, None),
                    Cell::new('e', 6, Color::Black, None),
                    Cell::new('f', 6, Color::Black, None),
                    Cell::new('g', 6, Color::Black, None),
                    Cell::new('h', 6, Color::Black, None),
                ],
                vec![
                    Cell::new('a', 5, Color::Black, None),
                    Cell::new('b', 5, Color::Black, None),
                    Cell::new('c', 5, Color::Black, None),
                    Cell::new('d', 5, Color::Black, None),
                    Cell::new('e', 5, Color::Black, None),
                    Cell::new('f', 5, Color::Black, None),
                    Cell::new('g', 5, Color::Black, None),
                    Cell::new('h', 5, Color::Black, None),
                ],
                vec![
                    Cell::new('a', 4, Color::Black, None),
                    Cell::new('b', 4, Color::Black, None),
                    Cell::new('c', 4, Color::Black, None),
                    Cell::new('d', 4, Color::Black, None),
                    Cell::new('e', 4, Color::Black, None),
                    Cell::new('f', 4, Color::Black, None),
                    Cell::new('g', 4, Color::Black, None),
                    Cell::new('h', 4, Color::Black, None),
                ],
                vec![
                    Cell::new('a', 3, Color::Black, None),
                    Cell::new('b', 3, Color::Black, None),
                    Cell::new('c', 3, Color::Black, None),
                    Cell::new('d', 3, Color::Black, None),
                    Cell::new('e', 3, Color::Black, None),
                    Cell::new('f', 3, Color::Black, None),
                    Cell::new('g', 3, Color::Black, None),
                    Cell::new('h', 3, Color::Black, None),
                ],
                vec![
                    Cell::new('a', 2, Color::Black, None),
                    Cell::new('b', 2, Color::Black, None),
                    Cell::new('c', 2, Color::Black, None),
                    Cell::new('d', 2, Color::Black, None),
                    Cell::new('e', 2, Color::Black, None),
                    Cell::new('f', 2, Color::Black, None),
                    Cell::new('g', 2, Color::Black, None),
                    Cell::new('h', 2, Color::Black, None),
                ],
                vec![
                    Cell::new('a', 1, Color::Black, None),
                    Cell::new('b', 1, Color::Black, None),
                    Cell::new('c', 1, Color::Black, None),
                    Cell::new('d', 1, Color::Black, None),
                    Cell::new('e', 1, Color::Black, None),
                    Cell::new('f', 1, Color::Black, None),
                    Cell::new('g', 1, Color::Black, None),
                    Cell::new('h', 1, Color::Black, None),
                ],
            ],
            board_size: 8,
        };

        Ok(board)
    }

    pub fn load_from_file(board_name: &str) -> Result<ChessBoard, Box<dyn std::error::Error>> {
        let board_string = match board_name {
            "game_start" => include_str!("../../boards/game_start.fen"),
            "empty_board" => include_str!("../../boards/empty_board.fen"),
            _ => return Err(Box::from("Invalid option for board_name")),
        };

        let board = ChessBoard::load_from_fen(board_string.to_string())?;

        Ok(board)
    }

    /// Creates a `Board` object from a Fen string. NOTE: assumes the game section of the fen has been removed
    pub fn load_from_fen(fen: String) -> Result<ChessBoard, Box<dyn std::error::Error>> {
        let mut board = ChessBoard::load_empty_board()?;
        let sections: Vec<Vec<char>> = fen
            .split('/')
            .map(|s| s.to_string().chars().collect())
            .collect();

        let mut cells = board.all_cells_mut();
        let mut cell_iter = cells.iter_mut();

        for c_vec in sections {
            for c in c_vec {
                if c.is_numeric() {
                    let num_to_skip = c
                        .to_digit(10)
                        .expect("Unable to convert the num to skip to i32");
                    for _ in 0..num_to_skip {
                        cell_iter.next();
                    }
                } else {
                    let cell = cell_iter
                        .next()
                        .expect("The iterator really shouldnt be empty :(");
                    cell.space
                        .get_or_insert(piece_from_fen_char(c, cell.x, cell.y)?);
                }
            }
        }

        Ok(board)
    }

    pub fn board_fen_section(&self) -> String {
        let mut fen = "".to_string();

        //construct the piece position portion
        for row in &self.squares {
            let mut num_empty_squares: i32 = 0;
            for cell in row {
                if let Some(piece) = &cell.space {
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
            fen.push('/');
        }

        fen[0..fen.len() - 1].to_string()
    }
}

fn piece_from_fen_char(
    c: char,
    x: char,
    y: i32,
) -> Result<Box<dyn AChessPiece>, Box<dyn std::error::Error>> {
    match c {
        'p' => Ok(Box::from(Pawn {
            piece_type: PieceType::Pawn,
            color: Color::Black,
            current_position: Coordinate { x, y },
        })),
        'r' => Ok(Box::from(Rook {
            piece_type: PieceType::Rook,
            color: Color::Black,
            current_position: Coordinate { x, y },
        })),
        'n' => Ok(Box::from(Knight {
            piece_type: PieceType::Knight,
            color: Color::Black,
            current_position: Coordinate { x, y },
        })),
        'b' => Ok(Box::from(Bishop {
            piece_type: PieceType::Bishop,
            color: Color::Black,
            current_position: Coordinate { x, y },
        })),
        'k' => Ok(Box::from(King {
            piece_type: PieceType::King,
            color: Color::Black,
            current_position: Coordinate { x, y },
        })),
        'q' => Ok(Box::from(Queen {
            piece_type: PieceType::Queen,
            color: Color::Black,
            current_position: Coordinate { x, y },
        })),
        'P' => Ok(Box::from(Pawn {
            piece_type: PieceType::Pawn,
            color: Color::White,
            current_position: Coordinate { x, y },
        })),
        'R' => Ok(Box::from(Rook {
            piece_type: PieceType::Rook,
            color: Color::White,
            current_position: Coordinate { x, y },
        })),
        'N' => Ok(Box::from(Knight {
            piece_type: PieceType::Knight,
            color: Color::White,
            current_position: Coordinate { x, y },
        })),
        'B' => Ok(Box::from(Bishop {
            piece_type: PieceType::Bishop,
            color: Color::White,
            current_position: Coordinate { x, y },
        })),
        'K' => Ok(Box::from(King {
            piece_type: PieceType::King,
            color: Color::White,
            current_position: Coordinate { x, y },
        })),
        'Q' => Ok(Box::from(Queen {
            piece_type: PieceType::Queen,
            color: Color::White,
            current_position: Coordinate { x, y },
        })),
        _ => Err(Box::from("Invalid char: {}")),
    }
}
