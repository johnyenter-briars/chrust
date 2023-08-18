use crate::board::{
    chessboard::ChessBoard,
    color::{self, Color},
    coordinate::Coordinate,
    piecetype::PieceType,
};

use super::{chesspiece::ChessPiece, down_by, enemy_occupied, friendly_occupied, up_by};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Pawn {
    pub color: Color,
    pub current_position: Coordinate,
    pub piece_type: PieceType,
}

impl ChessPiece for Pawn {
    fn available_moves(&self, board: &ChessBoard) -> Vec<Coordinate> {
        let mut possible_coordinates: Vec<Coordinate> = Vec::new();

        let target_direction: fn(Coordinate, u32) -> Coordinate = match self.color {
            Color::White => up_by,
            Color::Black => down_by,
        };

        let num_moves_possible = match (self.color, self.current_position.y) {
            (Color::White, 2) => 2,
            (Color::Black, 7) => 2,
            _ => 1,
        };

        for position in 1..num_moves_possible + 1 {
            let possible_position = target_direction(self.current_position, position);

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
        let possible_enemy_positions: Vec<Coordinate> = if self.color == color::Color::White {
            vec![
                self.current_position.diagonal_up_right_by(1),
                self.current_position.diagonal_up_left_by(1),
            ]
        } else {
            vec![
                self.current_position.diagonal_down_left_by(1),
                self.current_position.diagonal_down_right_by(1),
            ]
        }
        .into_iter()
        .filter(|coord| coord.is_valid(board)) //filter out the ones that arent valid (i.e, diagonal left might be off the map)
        .collect();

        for coord in possible_enemy_positions {
            if enemy_occupied(coord, self.color, board) {
                possible_coordinates.push(coord)
            };
        }

        possible_coordinates
    }
    fn str(&self) -> &str {
        match self.color {
            Color::White => "\u{2659}",
            Color::Black => "\u{265F}",
        }
    }
    fn fen_code(&self) -> &str {
        match self.color {
            Color::White => "P",
            Color::Black => "p",
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
