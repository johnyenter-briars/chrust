use crate::board::{
    chessboard::ChessBoard, color::Color, coordinate::Coordinate, movetype::MoveType,
};

use super::{
    chesspiece::ChessPiece, diagonal_down_left_by, diagonal_down_right_by, diagonal_up_left_by,
    diagonal_up_right_by, enemy_occupied, friendly_occupied, piecetype::PieceType,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Bishop {
    pub color: Color,
    pub current_position: Coordinate,
    pub piece_type: PieceType,
}

impl ChessPiece for Bishop {
    fn available_moves(&self, board: &ChessBoard) -> Vec<(Coordinate, MoveType)> {
        let mut possible_coordinates: Vec<(Coordinate, MoveType)> = Vec::new();

        let possible_directions: Vec<fn(Coordinate, i32) -> Coordinate> = vec![
            diagonal_up_left_by,
            diagonal_up_right_by,
            diagonal_down_right_by,
            diagonal_down_left_by,
        ];

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
            .into_iter()
            .filter(|coord| coord.0.is_valid(board))
            .collect()
    }
    fn str(&self) -> &str {
        match self.color {
            Color::White => "\u{2657}",
            Color::Black => "\u{265D}",
        }
    }
    fn fen_code(&self) -> &str {
        match self.color {
            Color::White => "B",
            Color::Black => "b",
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
