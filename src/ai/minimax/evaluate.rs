use crate::board::cell::{color::Color, piecetype::PieceType};

use super::boardstate::BoardState;

pub fn evaluate(board_state: &BoardState) -> i32{
    let pieces = board_state.board.get_all_pieces();
    pieces.iter().map(|p| get_piece_score(p.piece_type, p.color)).sum()
}

fn get_piece_score(piece_type: PieceType, color: Color) -> i32 {
    match (piece_type, color) {
        (PieceType::Pawn, Color::White) => 10,
        (PieceType::Pawn, Color::Black) => -10,
        (PieceType::Rook, Color::White) => 50,
        (PieceType::Rook, Color::Black) => -50,
        (PieceType::Knight, Color::White) => 30,
        (PieceType::Knight, Color::Black) => -30,
        (PieceType::Bishop, Color::White) => 30,
        (PieceType::Bishop, Color::Black) => -30,
        (PieceType::Queen, Color::White) => 900,
        (PieceType::Queen, Color::Black) => -90,
        (PieceType::King, Color::White) => 90,
        (PieceType::King, Color::Black) => -900,
    }
}