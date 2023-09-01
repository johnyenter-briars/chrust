use std::fmt::Debug;

use crate::{
    board::{chessboard::ChessBoard, color::Color, coordinate::Coordinate, movetype::MoveType},
    ChrustResult,
};

use super::{bishop::Bishop, knight::Knight, piecetype::PieceType, queen::Queen, rook::Rook};

pub trait ChessPiece: Debug {
    fn available_moves(&self, board: &ChessBoard) -> Vec<(Coordinate, MoveType)>;
    fn fen_code(&self) -> &str;
    fn str(&self) -> &str;
    fn piece_type(&self) -> PieceType;
    fn coordinate(&self) -> Coordinate;
    fn color(&self) -> Color;
}

pub fn chess_piece_from_type(
    t: PieceType,
    color: Color,
    current_position: Coordinate,
) -> ChrustResult<Box<dyn ChessPiece>> {
    match t {
        PieceType::Rook => Ok(Box::from(Rook {
            piece_type: PieceType::Rook,
            color,
            current_position,
        })),
        PieceType::Knight => Ok(Box::from(Knight {
            piece_type: PieceType::Knight,
            color,
            current_position,
        })),
        PieceType::Bishop => Ok(Box::from(Bishop {
            piece_type: PieceType::Bishop,
            color,
            current_position,
        })),
        PieceType::Queen => Ok(Box::from(Queen {
            piece_type: PieceType::Queen,
            color,
            current_position,
        })),
        _ => Err(Box::from("Invalid piecetype")),
    }
}
