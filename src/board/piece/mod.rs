use super::{coordinate::Coordinate, color::Color, chessboard::Board};

pub mod pawn;
pub mod chesspiece;
pub mod achesspiece;
pub mod king;
pub mod queen;
pub mod bishop;
pub mod rook;
pub mod knight;

fn enemy_occupied(coordinate: Coordinate, current_piece_color: Color, board: &Board) -> bool {
    let cell = board.test_cell_at(coordinate);

    match cell {
        Some(c) => match c.space {
            Some(piece) => piece.color != current_piece_color,
            None => false,
        },
        None => false,
    }
}

fn friendly_occupied(coordinate: Coordinate, current_piece_color: Color, board: &Board) -> bool {
    let cell = board.test_cell_at(coordinate);

    match cell {
        Some(c) => match c.space {
            Some(piece) => piece.color == current_piece_color,
            None => false,
        },
        None => false,
    }
}

fn up_by(coordinate: Coordinate, ammount: i32) -> Coordinate {
    coordinate.up_by(ammount)
}

fn down_by(coordinate: Coordinate, ammount: i32) -> Coordinate {
    coordinate.down_by(ammount)
}

fn left_by(coordinate: Coordinate, ammount: i32) -> Coordinate {
    coordinate.left_by(ammount)
}

fn right_by(coordinate: Coordinate, ammount: i32) -> Coordinate {
    coordinate.right_by(ammount)
}

fn diagonal_up_left_by(coordinate: Coordinate, ammount: i32) -> Coordinate {
    coordinate.diagonal_up_left_by(ammount)
}

fn diagonal_up_right_by(coordinate: Coordinate, ammount: i32) -> Coordinate {
    coordinate.diagonal_up_right_by(ammount)
}

fn diagonal_down_left_by(coordinate: Coordinate, ammount: i32) -> Coordinate {
    coordinate.diagonal_down_left_by(ammount)
}

fn diagonal_down_right_by(coordinate: Coordinate, ammount: i32) -> Coordinate {
    coordinate.diagonal_down_right_by(ammount)
}
