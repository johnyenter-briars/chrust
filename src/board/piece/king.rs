use crate::board::{
    chessboard::ChessBoard, color::Color, coordinate::Coordinate, piecetype::PieceType,
};

use super::{
    chesspiece::ChessPiece, diagonal_down_left_by, diagonal_down_right_by, diagonal_up_left_by,
    diagonal_up_right_by, down_by, enemy_occupied, friendly_occupied, left_by, right_by, up_by,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct King {
    pub color: Color,
    pub current_position: Coordinate,
    pub piece_type: PieceType,
}

impl ChessPiece for King {
    fn available_moves(&self, board: &ChessBoard) -> Vec<Coordinate> {
        let mut possible_coordinates: Vec<Coordinate> = Vec::new();

        let possible_directions: Vec<fn(Coordinate, i32) -> Coordinate> = vec![
            up_by,
            down_by,
            left_by,
            right_by,
            diagonal_up_left_by,
            diagonal_up_right_by,
            diagonal_down_right_by,
            diagonal_down_left_by,
        ];

        for direction_func in possible_directions {
            for position in 1..2 {
                let possible_position = direction_func(self.current_position, position);

                if !possible_position.is_valid(board) {
                    continue;
                };

                if friendly_occupied(possible_position, self.color, board) {
                    break;
                } else if enemy_occupied(possible_position, self.color, board) {
                    possible_coordinates.push(possible_position);
                    break;
                } else {
                    possible_coordinates.push(possible_position);
                }
            }
        }

        // let current_fen = board.ge;

        possible_coordinates
    }
    fn str(&self) -> &str {
        match self.color {
            Color::White => "\u{2654}",
            Color::Black => "\u{265A}",
        }
    }
    fn fen_code(&self) -> &str {
        match self.color {
            Color::White => "K",
            Color::Black => "k",
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
