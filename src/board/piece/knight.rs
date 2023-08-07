use crate::board::{chessboard::ChessBoard, color::Color, coordinate::Coordinate, piecetype::PieceType};

use super::{achesspiece::AChessPiece, friendly_occupied};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Knight {
    pub color: Color,
    pub current_position: Coordinate,
    pub piece_type: PieceType,
}

impl AChessPiece for Knight {
    fn available_moves(&self, board: &ChessBoard) -> Vec<Coordinate> {
        let mut possible_coordinates: Vec<Coordinate> = Vec::new();

        let possible_directions = vec![
            self.current_position.up_by(1).left_by(2),
            self.current_position.up_by(2).left_by(1),
            self.current_position.up_by(2).right_by(1),
            self.current_position.up_by(1).right_by(2),
            self.current_position.down_by(1).right_by(2),
            self.current_position.down_by(2).right_by(1),
            self.current_position.down_by(2).left_by(1),
            self.current_position.down_by(1).left_by(2),
        ];

        for possible_position in possible_directions {
            if !possible_position.is_valid(board) {
                continue;
            };

            //TODO - this could be more expressive i think
            if friendly_occupied(possible_position, self.color, board) {
                continue;
            } else {
                possible_coordinates.push(possible_position);
            }
        }

        possible_coordinates
    }
    fn str(&self) -> &str {
        match self.color {
            Color::White => "\u{2658}",
            Color::Black => "\u{265E}",
        }
    }
    fn fen_code(&self) -> &str {
        match self.color {
            Color::White => "N",
            Color::Black => "n",
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
