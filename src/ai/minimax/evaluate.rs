use crate::board::cell::{color::Color, piecetype::PieceType};

use super::boardstate::BoardState;

pub fn evaluate(board_state: &BoardState) -> i32 {
    let pieces = board_state.board.get_all_pieces();
    pieces
        .iter()
        .map(|p| get_piece_score(p.piece_type, p.color))
        .sum()
}

//NOTE: this evaluation function assumes that the WHITE is the MIN player and BLACK is the MAX player
fn get_piece_score(piece_type: PieceType, color: Color) -> i32 {
    if color == Color::White {
        - get_piece_value(piece_type)
    } else {
        get_piece_value(piece_type)
    }
}

fn get_piece_value(piece_type: PieceType) -> i32 {
    match (piece_type) {
        PieceType::Pawn => 10,
        PieceType::Rook => 50,
        PieceType::Knight => 30,
        PieceType::Bishop => 30,
        PieceType::Queen => 90,
        PieceType::King => 900,
    }
}
