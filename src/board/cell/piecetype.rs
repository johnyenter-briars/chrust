use std::collections::HashSet;

use crate::{board::{chessboard::Board, coordinate::Coordinate}, chessmove::piecemove::Move};

use super::chesspiece::ChessPiece;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PieceType {
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
}   

impl PieceType {
    pub fn available_moves(&self, current_position: Coordinate, board: &Board, turn_num: i32) -> Vec<Coordinate> {
        let valid_moves: Vec<Coordinate> = match *self {
            PieceType::Pawn => { //pawns can only move one or two spaces - or attack diagonally 
                let possible_coordinates = if turn_num == 1 {
                    vec![current_position.forward_by(1), current_position.forward_by(2)]
                } else { 
                    vec![current_position.forward_by(1)]
                };
                
                filter_out_occupired_spaces(possible_coordinates, board)
            },
            PieceType::Rook => todo!(),
            PieceType::Knight => todo!(),
            PieceType::Bishop => todo!(),
            PieceType::Queen => todo!(),
            PieceType::King => todo!(),
        };

        valid_moves
    }
}

fn filter_out_occupired_spaces(possible_coordinates: Vec<Coordinate>, board: &Board) -> Vec<Coordinate> {
    let empty_spaces_on_board = board.get_empty_spaces();

    //should do something cute with intersection here - but my first try was 'cant do intersection on non primitative types' so oh well
    possible_coordinates.into_iter() //need to into_iter in order to coerce rust to NOT iterate over references
            .filter(|coord| 
                empty_spaces_on_board.iter().any(|board_coord| 
                    board_coord.x == coord.x && board_coord.y == coord.y))
            .collect()
}
