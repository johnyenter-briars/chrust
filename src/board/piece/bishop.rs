use crate::board::{chessboard::ChessBoard, color::Color, coordinate::Coordinate, piecetype::PieceType};

use super::{
    achesspiece::AChessPiece, diagonal_down_left_by,
    diagonal_down_right_by, diagonal_up_left_by, diagonal_up_right_by, enemy_occupied,
    friendly_occupied,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Bishop {
    pub color: Color,
    pub current_position: Coordinate,
    pub piece_type: PieceType,
}

impl AChessPiece for Bishop {
    fn available_moves(&self, board: &ChessBoard) -> Vec<Coordinate> {
        /*Algorithm:
            For each direction a bishop can move in:
                iterate up starting at the target piece in a diagonal line, if we hit an enemy, add that position, but dont go any further. If we hit a friendly piece, stop - dont add that position
        */
        let mut possible_coordinates: Vec<Coordinate> = Vec::new();

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
                    possible_coordinates.push(possible_position);
                    break;
                } else {
                    possible_coordinates.push(possible_position);
                }
            }
        }

        possible_coordinates
            .into_iter()
            .filter(|coord| coord.is_valid(board))
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
