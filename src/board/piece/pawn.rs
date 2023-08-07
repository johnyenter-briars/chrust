use crate::board::{chessboard::Board, color::{Color, self}, coordinate::Coordinate};

use super::{achesspiece::AChessPiece, chesspiece::ChessPiece, enemy_occupied};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Pawn(ChessPiece);

impl AChessPiece for Pawn {
    fn available_moves(
        &self,
        target_piece: &ChessPiece,
        current_position: Coordinate,
        board: &Board,
    ) -> Vec<Coordinate> {
        //pawns can only move one or two spaces (depending on turn num)
        let possible_coordinates = match (target_piece.color, current_position.y) {
            (Color::White, 2) => vec![current_position.up_by(1), current_position.up_by(2)],
            (Color::Black, 7) => {
                vec![current_position.down_by(1), current_position.down_by(2)]
            }
            (Color::White, _) => vec![current_position.up_by(1)],
            (Color::Black, _) => vec![current_position.down_by(1)],
        };

        //only include the ones where the enemy is NOT in front of
        let mut possible_coordinates: Vec<Coordinate> = possible_coordinates
            .into_iter()
            .filter(|coord| !enemy_occupied(coord.to_owned(), target_piece.color, board))
            .map(|coord| coord.to_owned())
            .collect();

        //Pawns can move diagonally if there's an enemy there
        let possible_enemy_positions: Vec<Coordinate> =
            if target_piece.color == color::Color::White {
                vec![
                    current_position.diagonal_up_right_by(1),
                    current_position.diagonal_up_left_by(1),
                ]
            } else {
                vec![
                    current_position.diagonal_down_left_by(1),
                    current_position.diagonal_down_right_by(1),
                ]
            }
            .into_iter()
            .filter(|coord| coord.is_valid(board)) //filter out the ones that arent valid (i.e, diagonal left might be off the map)
            .collect();

        for coord in possible_enemy_positions {
            if enemy_occupied(coord, target_piece.color, board) {
                possible_coordinates.push(coord)
            };
        }

        //only return the coordinates that are valid spaces (on the board)
        possible_coordinates
            .into_iter()
            .filter(|coord| coord.is_valid(board))
            .collect()
    }
    fn str(&self) -> &str {
        match self.0.color {
            Color::White => "\u{2659}",
            Color::Black => "\u{265F}",
        }
    }
    fn fen_code(&self) -> &str {
        match self.0.color {
            Color::White => "P",
            Color::Black => "p",
        }
    }
}
