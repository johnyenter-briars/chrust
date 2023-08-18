use super::{chessboard::ChessBoard, color::Color, coordinate::Coordinate};

pub mod bishop;
pub mod chesspiece;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

fn enemy_occupied(coordinate: Coordinate, current_piece_color: Color, board: &ChessBoard) -> bool {
    let cell = board.test_cell_at(coordinate);

    match cell {
        Some(c) => match &c.space {
            Some(piece) => piece.color() != current_piece_color,
            None => false,
        },
        None => false,
    }
}

fn friendly_occupied(
    coordinate: Coordinate,
    current_piece_color: Color,
    board: &ChessBoard,
) -> bool {
    let cell = board.test_cell_at(coordinate);

    match cell {
        Some(c) => match &c.space {
            Some(piece) => piece.color() == current_piece_color,
            None => false,
        },
        None => false,
    }
}

fn up_by(coordinate: Coordinate, ammount: u32) -> Coordinate {
    coordinate.up_by(ammount)
}

fn down_by(coordinate: Coordinate, ammount: u32) -> Coordinate {
    coordinate.down_by(ammount)
}

fn left_by(coordinate: Coordinate, ammount: u32) -> Coordinate {
    coordinate.left_by(ammount)
}

fn right_by(coordinate: Coordinate, ammount: u32) -> Coordinate {
    coordinate.right_by(ammount)
}

fn diagonal_up_left_by(coordinate: Coordinate, ammount: u32) -> Coordinate {
    coordinate.diagonal_up_left_by(ammount)
}

fn diagonal_up_right_by(coordinate: Coordinate, ammount: u32) -> Coordinate {
    coordinate.diagonal_up_right_by(ammount)
}

fn diagonal_down_left_by(coordinate: Coordinate, ammount: u32) -> Coordinate {
    coordinate.diagonal_down_left_by(ammount)
}

fn diagonal_down_right_by(coordinate: Coordinate, ammount: u32) -> Coordinate {
    coordinate.diagonal_down_right_by(ammount)
}
