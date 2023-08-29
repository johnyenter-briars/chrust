use crate::board::{
    chessboard::ChessBoard,
    color::Color,
    coordinate::Coordinate,
    movetype::MoveType,
};

use super::{
    chesspiece::ChessPiece, diagonal_down_left_by, diagonal_down_right_by, diagonal_up_left_by,
    diagonal_up_right_by, down_by, enemy_occupied, friendly_occupied, left_by, right_by, up_by, piecetype::PieceType,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct King {
    pub color: Color,
    pub current_position: Coordinate,
    pub piece_type: PieceType,
}

impl ChessPiece for King {
    fn available_moves(&self, board: &ChessBoard) -> Vec<(Coordinate, MoveType)> {
        let mut possible_coordinates: Vec<(Coordinate, MoveType)> = Vec::new();

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
                    possible_coordinates.push((possible_position, MoveType::CaptureOrMove));
                    break;
                } else {
                    possible_coordinates.push((possible_position, MoveType::CaptureOrMove));
                }
            }
        }

        if self.color == Color::White {
            if board.white_castle_kingside
                && board
                    .test_cell_at(Coordinate::new('f', 1))
                    .unwrap()
                    .space
                    .is_none()
                && board
                    .test_cell_at(Coordinate::new('g', 1))
                    .unwrap()
                    .space
                    .is_none()
            {
                possible_coordinates.push((Coordinate::new('g', 1), MoveType::Castle));
            }

            if board.white_castle_queenside
                && board
                    .test_cell_at(Coordinate::new('b', 1))
                    .unwrap()
                    .space
                    .is_none()
                && board
                    .test_cell_at(Coordinate::new('c', 1))
                    .unwrap()
                    .space
                    .is_none()
                && board
                    .test_cell_at(Coordinate::new('d', 1))
                    .unwrap()
                    .space
                    .is_none()
            {
                possible_coordinates.push((Coordinate::new('c', 1), MoveType::Castle));
            }
        } else {
            if board.black_castle_kingside
                && board
                    .test_cell_at(Coordinate::new('f', 8))
                    .unwrap()
                    .space
                    .is_none()
                && board
                    .test_cell_at(Coordinate::new('g', 8))
                    .unwrap()
                    .space
                    .is_none()
            {
                possible_coordinates.push((Coordinate::new('g', 8), MoveType::Castle));
            }

            if board.white_castle_queenside
                && board
                    .test_cell_at(Coordinate::new('b', 8))
                    .unwrap()
                    .space
                    .is_none()
                && board
                    .test_cell_at(Coordinate::new('c', 8))
                    .unwrap()
                    .space
                    .is_none()
                && board
                    .test_cell_at(Coordinate::new('d', 8))
                    .unwrap()
                    .space
                    .is_none()
            {
                possible_coordinates.push((Coordinate::new('c', 8), MoveType::Castle));
            }
        }

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
