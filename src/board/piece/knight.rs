use crate::board::{chessboard::Board, color::Color, coordinate::Coordinate};

use super::{achesspiece::AChessPiece, chesspiece::ChessPiece, friendly_occupied};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Knight(ChessPiece);

impl AChessPiece for Knight {
    fn available_moves(
        &self,
        board: &Board,
    ) -> Vec<Coordinate> {
        let mut possible_coordinates: Vec<Coordinate> = Vec::new();

        let possible_directions = vec![
            self.0.current_position.up_by(1).left_by(2),
            self.0.current_position.up_by(2).left_by(1),
            self.0.current_position.up_by(2).right_by(1),
            self.0.current_position.up_by(1).right_by(2),
            self.0.current_position.down_by(1).right_by(2),
            self.0.current_position.down_by(2).right_by(1),
            self.0.current_position.down_by(2).left_by(1),
            self.0.current_position.down_by(1).left_by(2),
        ];

        for possible_position in possible_directions {
            if !possible_position.is_valid(board) {
                continue;
            };

            //TODO - this could be more expressive i think
            if friendly_occupied(possible_position, self.0.color, board) {
                continue;
            } else {
                possible_coordinates.push(possible_position);
            }
        }

        possible_coordinates
    }
    fn str(&self) -> &str {
        match self.0.color {
            Color::White => "\u{2658}",
            Color::Black => "\u{265E}",
        }
    }
    fn fen_code(&self) -> &str {
        match self.0.color {
            Color::White => "N",
            Color::Black => "n",
        }
    }
}
