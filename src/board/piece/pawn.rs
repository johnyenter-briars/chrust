use crate::board::{
    chessboard::ChessBoard,
    color::{self, Color},
    coordinate::Coordinate,
    piecetype::PieceType,
};

use super::{chesspiece::ChessPiece, enemy_occupied};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Pawn {
    pub color: Color,
    pub current_position: Coordinate,
    pub piece_type: PieceType,
}

impl ChessPiece for Pawn {
    fn available_moves(&self, board: &ChessBoard) -> Vec<Coordinate> {
        //pawns can only move one or two spaces (depending on turn num)
        let possible_coordinates = match (self.color, self.current_position.y) {
            (Color::White, 2) => vec![
                self.current_position.up_by(1),
                self.current_position.up_by(2),
            ],
            (Color::Black, 7) => {
                vec![
                    self.current_position.down_by(1),
                    self.current_position.down_by(2),
                ]
            }
            (Color::White, _) => vec![self.current_position.up_by(1)],
            (Color::Black, _) => vec![self.current_position.down_by(1)],
        };

        //only include the ones where the enemy is NOT in front of
        let mut possible_coordinates: Vec<Coordinate> = possible_coordinates
            .into_iter()
            .filter(|coord| !enemy_occupied(coord.to_owned(), self.color, board))
            .map(|coord| coord.to_owned())
            .collect();

        //Pawns can move diagonally if there's an enemy there
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

        //only return the coordinates that are valid spaces (on the board)
        possible_coordinates
            .into_iter()
            .filter(|coord| coord.is_valid(board))
            .collect()
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
