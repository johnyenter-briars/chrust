use crate::board::{chessboard::Board, color::Color, coordinate::Coordinate};

use super::{
    achesspiece::AChessPiece, chesspiece::ChessPiece, down_by, enemy_occupied, friendly_occupied,
    left_by, right_by, up_by,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rook(ChessPiece);

impl AChessPiece for Rook {
    fn available_moves(&self, board: &Board) -> Vec<Coordinate> {
        /*Algorithm:
            For each direction a rook can move in:
                iterate up starting at the target piece in a diagonal line, if we hit an enemy, add that position, but dont go any further. If we hit a friendly piece, stop - dont add that position
        */
        let mut possible_coordinates: Vec<Coordinate> = Vec::new();

        let possible_directions: Vec<fn(Coordinate, i32) -> Coordinate> =
            vec![up_by, down_by, left_by, right_by];

        for direction_func in possible_directions {
            for position in 1..9 {
                let possible_position = direction_func(self.0.current_position, position);

                if !possible_position.is_valid(board) {
                    continue;
                };

                if friendly_occupied(possible_position, self.0.color, board) {
                    break;
                } else if enemy_occupied(possible_position, self.0.color, board) {
                    possible_coordinates.push(possible_position);
                    break;
                } else {
                    possible_coordinates.push(possible_position);
                }
            }
        }

        possible_coordinates
    }
    fn str(&self) -> &str {
        match self.0.color {
            Color::White => "\u{2656}",
            Color::Black => "\u{265C}",
        }
    }
    fn fen_code(&self) -> &str {
        match self.0.color {
            Color::White => "R",
            Color::Black => "r",
        }
    }
}
