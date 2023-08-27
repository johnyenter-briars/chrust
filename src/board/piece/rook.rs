use crate::board::{
    chessboard::ChessBoard,
    color::Color,
    coordinate::Coordinate,
    types::{MoveType, PieceType},
};

use super::{
    chesspiece::ChessPiece, down_by, enemy_occupied, friendly_occupied, left_by, right_by, up_by,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rook {
    pub color: Color,
    pub current_position: Coordinate,
    pub piece_type: PieceType,
}

impl ChessPiece for Rook {
    fn available_moves(&self, board: &ChessBoard) -> Vec<(Coordinate, MoveType)> {
        let mut possible_coordinates: Vec<(Coordinate, MoveType)> = Vec::new();

        let possible_directions: Vec<fn(Coordinate, i32) -> Coordinate> =
            vec![up_by, down_by, left_by, right_by];

        for direction_func in possible_directions {
            for position in 1..9 {
                let possible_position = direction_func(self.current_position, position);

                if !possible_position.is_valid(board) {
                    continue;
                };

                if friendly_occupied(possible_position, self.color, board) {
                    break;
                } else if enemy_occupied(possible_position, self.color, board) {
                    possible_coordinates.push((possible_position, MoveType::CaptureOrMove));
                    break;
                } else {
                    possible_coordinates.push((possible_position, MoveType::CaptureOrMove));
                }
            }
        }

        possible_coordinates
    }
    fn str(&self) -> &str {
        match self.color {
            Color::White => "\u{2656}",
            Color::Black => "\u{265C}",
        }
    }
    fn fen_code(&self) -> &str {
        match self.color {
            Color::White => "R",
            Color::Black => "r",
        }
    }
    fn piece_type(&self) -> PieceType {
        self.piece_type
    }
    fn coordinate(&self) -> Coordinate {
        self.current_position
    }
    fn color(&self) -> Color {
        self.color
    }
}
