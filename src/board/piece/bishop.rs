use crate::board::{chessboard::Board, color::Color, coordinate::Coordinate};

use super::{achesspiece::AChessPiece, chesspiece::ChessPiece, friendly_occupied, enemy_occupied, diagonal_up_left_by, diagonal_up_right_by, diagonal_down_right_by, diagonal_down_left_by};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Bishop(ChessPiece);

impl AChessPiece for Bishop {
    fn available_moves(
        &self,
        target_piece: &ChessPiece,
        current_position: Coordinate,
        board: &Board,
    ) -> Vec<Coordinate> {
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
                let possible_position = direction_func(current_position, position);

                if !possible_position.is_valid(board) {
                    continue;
                };

                if friendly_occupied(possible_position, target_piece.color, board) {
                    break;
                } else if enemy_occupied(possible_position, target_piece.color, board) {
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
        match self.0.color {
            Color::White => "\u{2657}",
            Color::Black => "\u{265D}",
        }
    }
    fn fen_code(&self) -> &str {
        match self.0.color {
            Color::White => "B",
            Color::Black => "b",
        }
    }
}
