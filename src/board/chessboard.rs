extern crate serde_derive;
extern crate serde_json;

use crate::board::piece::pawn::Pawn;
use crate::chessmove::piecemove::PieceMove;
use crate::ChrustResult;
// use crate::board::cell::piecetype::PieceType;
// use crate::chessmove::piecemove::{PieceMove};
use crate::player::humanplayer::HumanPlayer;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::vec;

// use crate::board::cell::chesspiece::ChessPiece;

use super::cell::Cell;
use super::color::Color;
use super::coordinate::Coordinate;
use super::piece::bishop::Bishop;
use super::piece::chesspiece::ChessPiece;
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
    fn all_cells_mut_refcell(&mut self) -> Vec<RefCell<&mut Cell>> {
        self.squares
            .iter_mut()
            .flatten()
            .map(|c| RefCell::new(c))
            .collect()
    }

    pub fn print_to_screen(&self, configuration_name: &str) {
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
    pub fn piece(&self, x: char, y: i32) -> ChrustResult<&Box<dyn ChessPiece>> {
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
    ) -> ChrustResult<Vec<Coordinate>> {
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
    ) -> ChrustResult<Vec<Coordinate>> {
        let target_piece = self.piece(current_position.x, current_position.y)?;

        if target_piece.color() != color {
            return Err(Box::from("Thats not your piece!"));
        }

        Ok(target_piece.available_moves(self))
    }

    pub fn all_possible_moves(&self, color: Color) -> ChrustResult<Vec<PieceMove>> {
        let cells = self.cells_with_pieces_with_color(color);

        let mut possible_moves: Vec<PieceMove> = Vec::new();

        for cell in cells {
            let current_position = Coordinate::new(cell.x, cell.y);
            let piece_at_position = self.piece(current_position.x, current_position.y)?;

            let coord_choices = self.possible_moves(current_position, color)?;

            let mut possible_moves_for_piece: Vec<PieceMove> = coord_choices
                .iter()
                .map(|coord| PieceMove::new(piece_at_position, current_position, *coord))
                .collect();

            possible_moves.append(&mut possible_moves_for_piece);
        }

        Ok(possible_moves)
    }

    // pub fn export_fen(&self) -> String {
    //     let cells = self.all_cells();
    //     let mut fen = String::new();
    //     let mut empty_count = 0;

    //     for cell in cells.iter() {
    //         match &cell.space {
    //             Some(piece) => {
    //                 if empty_count > 0 {
    //                     fen.push_str(&empty_count.to_string());
    //                     empty_count = 0;
    //                 }
    //                 let fen_code = piece.fen_code();
    //                 fen.push_str(fen_code);
    //             }
    //             None => {
    //                 empty_count += 1;
    //             }
    //         }

    //         if cell.x == 'h' {
    //             if empty_count > 0 {
    //                 fen.push_str(&empty_count.to_string());
    //             }
    //             if cell.y > 1 {
    //                 fen.push('/');
    //             }
    //             empty_count = 0;
    //         }
    //     }

    //     fen
    // }

    pub fn apply_action(&self, action: &PieceMove) -> ChrustResult<ChessBoard> {
        let fen = self.board_fen_section();
        self.print_to_screen(&"request");
        let mut new_board = ChessBoard::load_from_fen(fen)?;
        new_board.move_piece(action.from, action.to);
        new_board.print_to_screen(&"response");
        panic!("stopping here");
        Ok(new_board)
    }

    pub fn pieces_of_color(&self, color: Color) -> Vec<Box<dyn ChessPiece>> {
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
    ) -> Option<Box<&dyn ChessPiece>> {
        let pieces = self.pieces_of_color(color);
        todo!();
        // pieces.into_iter().find(|x| x.piece_type() == piece_type)
    }

    pub fn move_piece(&mut self, from: Coordinate, to: Coordinate) {
        // let mut cells: Vec<&mut Cell> = self.all_cells_mut();
        let cells = self.all_cells_mut_refcell();
        let from_index = cells
            .iter()
            .position(|c| c.borrow_mut().x == from.x && c.borrow_mut().y == from.y);

        if let Some(from_index) = from_index {
            // Extract the target cell using the index
            // let target_cell = &cells[from_index];

            // Find the target cell index
            let to_index = cells
                .iter()
                .position(|c| c.borrow_mut().x == to.x && c.borrow_mut().y == to.y);

            if let Some(to_index) = to_index {
                // Swap the chess pieces
                let one = &mut cells[from_index].borrow_mut().space;
                let two = &mut cells[to_index].borrow_mut().space;
                std::mem::swap(one, two);
                // if let (Some(one), Some(two)) = () {}
                *one = None;
            }
        }

        //1: get the reference to the piece we're going to move
        // let cells: Vec<&mut Cell> = self.all_cells_mut();
        // let binding: Vec<&&mut Cell> = cells
        //     .iter()
        //     .filter(|c| c.x == from.x && c.y == from.y)
        //     // .map(|c| c.space.expect("This space should not be empty!"))
        //     .collect::<Vec<&&mut Cell>>();

        // let target_cell: &&&mut Cell = binding
        //     .first()
        //     .expect("There should be a space at the 'from' location");

        // // let foo = *target_piece;

        // //2: move the reference to the stop we're moving to
        // for cell in &cells {
        //     if cell.x == to.x && cell.y == to.y {
        //         //we can safely assume there is NO piece at this location
        //         //since the from x and y are given to us by the set of possible moves
        //         //calculated ahead of time

        //         // cell.space = Some(foo);
        //         // std::mem::swap(&mut cell.space, &mut target_cell.space);
        //         std::mem::swap(&mut cell.space, &mut target_cell.space);
        //     }
        // }

        //3: set the current cell to nothing
        // self.space_to_empty(from.x, from.y);
    }
    fn swap_pieces(cell1: &mut Cell, cell2: &mut Cell) {
        // Swap the chess pieces in the cells
        std::mem::swap(&mut cell1.space, &mut cell2.space);
    }

    pub fn load_empty_board() -> ChrustResult<ChessBoard> {
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

    pub fn load_from_file(board_name: &str) -> ChrustResult<ChessBoard> {
        let board_string = match board_name {
            "game_start" => include_str!("../../boards/game_start.fen"),
            "empty_board" => include_str!("../../boards/empty_board.fen"),
            _ => return Err(Box::from("Invalid option for board_name")),
        };

        let board = ChessBoard::load_from_fen(board_string.to_string())?;

        Ok(board)
    }

    /// Creates a `Board` object from a Fen string. NOTE: assumes the game section of the fen has been removed
    pub fn load_from_fen(fen: String) -> ChrustResult<ChessBoard> {
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

fn piece_from_fen_char(c: char, x: char, y: i32) -> ChrustResult<Box<dyn ChessPiece>> {
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
