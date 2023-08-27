extern crate serde_derive;
extern crate serde_json;

use super::cell::Cell;
use super::color::Color;
use super::coordinate::Coordinate;
use super::piece::bishop::Bishop;
use super::piece::chesspiece::ChessPiece;
use super::piece::king::King;
use super::piece::knight::Knight;
use super::piece::queen::Queen;
use super::piece::rook::Rook;
use super::types::{MoveType, PieceType};
use crate::board::piece::pawn::Pawn;
use crate::chessmove::piecemove::PieceMove;
use crate::player::humanplayer::HumanPlayer;
use crate::ChrustResult;
use core::panic;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ChessBoard {
    pub squares: Vec<Vec<Cell>>,
    pub board_size: i32,
    pub fen: String,
    pub side_to_move: Color,
    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,
    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,
    pub en_passant_target: Option<Coordinate>,
    pub half_move_clock: u32,
    pub full_move_clock: u32,
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
            .map(RefCell::new)
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

        todo!("I literally don't care about this functionality");
        // Ok(target_piece.available_moves(self))
    }

    pub fn all_possible_moves(&self, color: Color) -> ChrustResult<Vec<PieceMove>> {
        let cells = self.cells_with_pieces_with_color(color);

        let mut possible_moves: Vec<PieceMove> = Vec::new();

        for cell in cells {
            let current_position = Coordinate::new(cell.x, cell.y);
            let piece_at_position = self.piece(current_position.x, current_position.y)?;

            let coord_choices = piece_at_position.available_moves(self);

            let mut possible_moves_for_piece: Vec<PieceMove> = coord_choices
                .iter()
                .map(|coord| {
                    PieceMove::new(piece_at_position, current_position, coord.0, color, coord.1)
                })
                .collect();

            possible_moves.append(&mut possible_moves_for_piece);
        }

        Ok(possible_moves)
    }

    pub fn apply_action(&self, action: &PieceMove) -> ChrustResult<ChessBoard> {
        let fen = self.fen();
        // self.print_to_screen("request");
        let mut new_board = ChessBoard::load_from_fen(&fen)?;
        new_board.move_piece(action)?;
        // new_board.print_to_screen("response");
        Ok(new_board)
    }

    pub fn pieces_of_color(&self, color: Color) -> Vec<&Box<dyn ChessPiece>> {
        self.all_cells()
            .iter()
            .flat_map(|c| &c.space)
            .filter(|c| c.color() == color)
            .collect::<Vec<&Box<dyn ChessPiece>>>()
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
    ) -> Option<&Box<dyn ChessPiece>> {
        self.pieces_of_color(color)
            .into_iter()
            .find(|x| x.piece_type() == piece_type)
    }

    pub fn move_piece(&mut self, action: &PieceMove) -> ChrustResult<()> {
        let from = action.from;
        let to = action.to;

        //check for en passent
        self.en_passant_target = match (
            action.chess_piece.piece_type(),
            action.chess_piece.color(),
            action.from.y,
            action.to.y,
        ) {
            (PieceType::Pawn, Color::White, 2, 4) => Some(Coordinate::new(action.from.x, 3)),
            (PieceType::Pawn, Color::Black, 7, 5) => Some(Coordinate::new(action.from.x, 6)),
            _ => None,
        };

        //actually move the piece. This is really bad - and i duplidcated this code. And I'm really sorry. But it's 6AM, and I'm super sad cause the swimming part of the triathlon this morning got canceled. So I did all that training for nothing.
        let cells = self.all_cells_mut_refcell();
        let from_index = cells
            .iter()
            .position(|c| c.borrow_mut().x == from.x && c.borrow_mut().y == from.y)
            .expect(&format!("No from index: {:?}", from));

        let to_index = cells
            .iter()
            .position(|c| c.borrow_mut().x == to.x && c.borrow_mut().y == to.y)
            .expect(&format!("No to index: {:?}", to));

        {
            let one = &mut cells[from_index].borrow_mut().space;
            let two = &mut cells[to_index].borrow_mut().space;
            std::mem::swap(one, two);
            *one = None;
        }

        if action.move_type == MoveType::EnPassant {
            let pawn_to_remove_index = match action.color_moving {
                Color::White => cells
                    .iter()
                    .position(|c| c.borrow_mut().x == to.x && c.borrow_mut().y == to.y - 1)
                    .expect(&format!("No to index: {:?}", to)),
                Color::Black => cells
                    .iter()
                    .position(|c| c.borrow_mut().x == to.x && c.borrow_mut().y == to.y + 1)
                    .expect(&format!("No to index: {:?}", to)),
            };

            cells[pawn_to_remove_index].borrow_mut().space = None;
        }

        //assuming we're moving the king currently
        if action.move_type == MoveType::Castle {
            if to.x == 'g' {
                let from_index_for_rook = cells
                    .iter()
                    .position(|c| c.borrow_mut().x == 'h' && c.borrow_mut().y == from.y)
                    .expect(&format!("No from index: {:?}", from));

                let to_index_for_rook = cells
                    .iter()
                    .position(|c| c.borrow_mut().x == 'f' && c.borrow_mut().y == to.y)
                    .expect(&format!("No to index: {:?}", to));

                {
                    let one = &mut cells[from_index_for_rook].borrow_mut().space;
                    let two = &mut cells[to_index_for_rook].borrow_mut().space;
                    std::mem::swap(one, two);
                    *one = None;
                }
            }

            if to.x == 'c' {
                let from_index_for_rook = cells
                    .iter()
                    .position(|c| c.borrow_mut().x == 'a' && c.borrow_mut().y == from.y)
                    .expect(&format!("No from index: {:?}", from));

                let to_index_for_rook = cells
                    .iter()
                    .position(|c| c.borrow_mut().x == 'd' && c.borrow_mut().y == to.y)
                    .expect(&format!("No to index: {:?}", to));

                {
                    let one = &mut cells[from_index_for_rook].borrow_mut().space;
                    let two = &mut cells[to_index_for_rook].borrow_mut().space;
                    std::mem::swap(one, two);
                    *one = None;
                }
            }

            if action.color_moving == Color::White {
                self.white_castle_kingside = false;
                self.white_castle_queenside = false;
            } else {
                self.black_castle_kingside = false;
                self.black_castle_queenside = false;
            }
        } else {
            //check for castling rights
            self.update_castling_rights(action)?;
        }

        // Update who's turn it is to move
        self.side_to_move = match action.color_moving {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        //TODO implement halfmove clock
        self.half_move_clock = self.half_move_clock;

        self.full_move_clock = match action.color_moving {
            Color::White => self.full_move_clock,
            Color::Black => self.full_move_clock + 1,
        };

        Ok(())
    }

    pub fn update_castling_rights(&mut self, action: &PieceMove) -> ChrustResult<()> {
        self.white_castle_kingside = !self.white_castle_kingside
            && !(action.chess_piece.piece_type() == PieceType::King
                && action.chess_piece.color() == Color::White);
        self.white_castle_queenside = !self.white_castle_queenside
            && !(action.chess_piece.piece_type() == PieceType::King
                && action.chess_piece.color() == Color::White);

        self.white_castle_kingside = !self.white_castle_kingside
            && !(action.chess_piece.piece_type() == PieceType::Rook
                && action.chess_piece.color() == Color::White
                && action.from.x == 'h');

        self.white_castle_queenside = !self.white_castle_queenside
            && !(action.chess_piece.piece_type() == PieceType::Rook
                && action.chess_piece.color() == Color::White
                && action.from.x == 'a');

        self.black_castle_kingside = !self.black_castle_kingside
            && !(action.chess_piece.piece_type() == PieceType::King
                && action.chess_piece.color() == Color::Black);
        self.black_castle_queenside = !self.black_castle_queenside
            && !(action.chess_piece.piece_type() == PieceType::King
                && action.chess_piece.color() == Color::Black);

        self.black_castle_kingside = !self.black_castle_kingside
            && !(action.chess_piece.piece_type() == PieceType::Rook
                && action.chess_piece.color() == Color::Black
                && action.from.x == 'h');

        self.black_castle_queenside = !self.black_castle_queenside
            && !(action.chess_piece.piece_type() == PieceType::Rook
                && action.chess_piece.color() == Color::Black
                && action.from.x == 'a');

        Ok(())
    }

    pub fn load_empty_board(
        side_to_move: Color,
        fen: String,
        white_castle_queenside: bool,
        white_castle_kingside: bool,
        black_castle_queenside: bool,
        black_castle_kingside: bool,
        en_passant_target: Option<Coordinate>,
        half_move_clock: u32,
        full_move_clock: u32,
    ) -> ChrustResult<ChessBoard> {
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
            fen,
            side_to_move,
            white_castle_queenside,
            white_castle_kingside,
            black_castle_queenside,
            black_castle_kingside,
            en_passant_target,
            half_move_clock,
            full_move_clock,
        };

        Ok(board)
    }

    pub fn load_from_file(board_name: &str) -> ChrustResult<ChessBoard> {
        let board_string = match board_name {
            "game_start" => include_str!("../../boards/game_start.fen"),
            "empty_board" => include_str!("../../boards/empty_board.fen"),
            _ => return Err(Box::from("Invalid option for board_name")),
        };

        let board = ChessBoard::load_from_fen(board_string)?;

        Ok(board)
    }

    pub fn full_fen() -> String {
        String::new()
    }

    pub fn load_from_fen(fen: &str) -> ChrustResult<ChessBoard> {
        let split = fen.split(' ').collect::<Vec<&str>>();

        let turn_to_move = match split
            .get(1)
            .unwrap_or_else(|| panic!("fen is improperly formatted! {}", fen))
        {
            &"w" => Color::White,
            &"b" => Color::Black,
            invalid_option => {
                return Err(Box::from(format!(
                    "Invalid option for 'turn to move': {}",
                    invalid_option
                )))
            }
        };

        let castling_rights = split
            .get(2)
            .unwrap_or_else(|| panic!("fen is improperly formatted! {}", fen));

        let white_castle_queenside = castling_rights.contains('Q');
        let white_castle_kingside = castling_rights.contains('K');
        let black_castle_kingside = castling_rights.contains('k');
        let black_castle_queenside = castling_rights.contains('q');

        let en_passant_target: Option<Coordinate> = match split
            .get(3)
            .unwrap_or_else(|| panic!("fen is improperly formatted! {}", fen))
        {
            &"-" => None,
            s => {
                let chars: Vec<char> = s.chars().collect();
                Some(Coordinate::new(chars[0], chars[1] as i32 - '0' as i32)) //sometimes you gotta go maverick
            }
        };

        let half_move_clock = split
            .get(4)
            .unwrap_or_else(|| panic!("fen is improperly formatted! {}", fen))
            .parse::<u32>()?;

        let full_move_clock = split
            .get(5)
            .unwrap_or_else(|| panic!("fen is improperly formatted! {}", fen))
            .parse::<u32>()?;

        let board_section = split
            .first()
            .unwrap_or_else(|| panic!("fen is improperly formatted! {}", fen));

        let mut board = ChessBoard::load_empty_board(
            turn_to_move,
            fen.to_string(),
            white_castle_queenside,
            white_castle_kingside,
            black_castle_queenside,
            black_castle_kingside,
            en_passant_target,
            half_move_clock,
            full_move_clock,
        )?;

        let sections: Vec<Vec<char>> = board_section
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

    pub fn fen(&self) -> String {
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

        let board_section = fen[0..fen.len() - 1].to_string();

        let en_passant_target = match self.en_passant_target {
            Some(s) => format!("{}{}", s.x, s.y),
            None => "-".to_string(),
        };

        format!(
            "{} {} {}{}{}{} {} {} {}",
            board_section,
            match self.side_to_move {
                Color::Black => "b",
                Color::White => "w",
            },
            if self.white_castle_kingside { "K" } else { "" },
            if self.white_castle_queenside { "Q" } else { "" },
            if self.black_castle_kingside { "k" } else { "" },
            if self.black_castle_queenside { "q" } else { "" },
            en_passant_target,
            self.half_move_clock,
            self.full_move_clock
        )
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
