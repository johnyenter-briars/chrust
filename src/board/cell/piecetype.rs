use std::{collections::HashSet, thread::current};

use crate::{
    board::{chessboard::Board, coordinate::Coordinate},
    chessmove::piecemove::Move,
};

use super::{chesspiece::ChessPiece, color::Color};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PieceType {
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
}

//TODO - refac this - should probably be impl ChessPiece instead
impl PieceType {
    pub fn available_moves(
        &self,
        target_piece: &ChessPiece,
        current_position: Coordinate,
        board: &Board,
        turn_num: i32,
    ) -> Vec<Coordinate> {
        let valid_moves: Vec<Coordinate> = match *self {
            PieceType::Pawn => {
                //pawns can only move one or two spaces (depending on turn num)
                let possible_coordinates = if turn_num == 1 {
                    vec![current_position.up_by(1), current_position.up_by(2)]
                } else {
                    vec![current_position.up_by(1)]
                };

                //only include the ones where the enemy is NOT in front of
                let mut possible_coordinates: Vec<Coordinate> = possible_coordinates
                    .into_iter()
                    .filter(|coord| !enemy_occupied(coord.to_owned(), target_piece.color, board))
                    .map(|coord| coord.to_owned())
                    .collect();

                //Pawns can move diagonally if there's an enemy there
                let possible_enemy_positions: Vec<Coordinate> = vec![
                    current_position.diagonal_up_right_by(1),
                    current_position.diagonal_up_left_by(1),
                ]
                .into_iter()
                .filter(|coord| coord.is_valid()) //filter out the ones that arent valid (i.e, diagonal left might be off the map)
                .collect();

                for coord in possible_enemy_positions {
                    if enemy_occupied(coord, target_piece.color, board) {
                        possible_coordinates.push(coord)
                    };
                }

                //only return the coordinates that are valid spaces (on the board)
                possible_coordinates
                    .into_iter()
                    .filter(|coord| coord.is_valid())
                    .collect()
            }
            PieceType::Rook => {
                /*Algorithm:
                    For each direction a rook can move in:
                        iterate up starting at the target piece in a diagonal line, if we hit an enemy, add that position, but dont go any further. If we hit a friendly piece, stop - dont add that position
                */
                let mut possible_coordinates: Vec<Coordinate> = Vec::new();

                let possible_directions: Vec<fn(Coordinate, i32) -> Coordinate> =
                    vec![up_by, down_by, left_by, right_by];

                for direction_func in possible_directions {
                    for position in (1..9).into_iter() {
                        let possible_position = direction_func(current_position, position);

                        if !possible_position.is_valid() {
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
            }
            PieceType::Knight => {
                let mut possible_coordinates: Vec<Coordinate> = Vec::new();

                let possible_directions = vec![
                    current_position.up_by(1).left_by(2),
                    current_position.up_by(2).left_by(1),
                    current_position.up_by(2).right_by(1),
                    current_position.up_by(1).right_by(2),
                    current_position.down_by(1).right_by(2),
                    current_position.down_by(2).right_by(1),
                    current_position.down_by(2).left_by(1),
                    current_position.down_by(1).left_by(2),
                ];

                for possible_position in possible_directions {
                    if !possible_position.is_valid() {
                        continue;
                    };

                    //TODO - this could be more expressive i think
                    if friendly_occupied(possible_position, target_piece.color, board) {
                        continue;
                    } else if enemy_occupied(possible_position, target_piece.color, board) {
                        possible_coordinates.push(possible_position);
                    } else {
                        possible_coordinates.push(possible_position);
                    }
                }

                possible_coordinates
            }
            PieceType::Bishop => {
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
                    for position in (1..9).into_iter() {
                        let possible_position = direction_func(current_position, position);

                        if !possible_position.is_valid() {
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
                    .filter(|coord| coord.is_valid())
                    .collect()
            }
            PieceType::Queen => {
                let mut possible_coordinates: Vec<Coordinate> = Vec::new();

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
                    for position in (1..9).into_iter() {
                        let possible_position = direction_func(current_position, position);

                        if !possible_position.is_valid() {
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
            }
            PieceType::King => {
                let mut possible_coordinates: Vec<Coordinate> = Vec::new();

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
                    for position in (1..2).into_iter() {
                        let possible_position = direction_func(current_position, position);

                        if !possible_position.is_valid() {
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
            }
        };

        valid_moves
    }
}

fn filter_out_occupired_spaces(
    possible_coordinates: Vec<Coordinate>,
    board: &Board,
) -> Vec<Coordinate> {
    let empty_spaces_on_board = board.get_empty_spaces();

    //should do something cute with intersection here - but my first try was 'cant do intersection on non primitative types' so oh well
    possible_coordinates
        .into_iter() //need to into_iter in order to coerce rust to NOT iterate over references
        .filter(|coord| {
            empty_spaces_on_board
                .iter()
                .any(|board_coord| board_coord.x == coord.x && board_coord.y == coord.y)
        })
        .collect()
}

fn enemy_occupied(coordinate: Coordinate, current_piece_color: Color, board: &Board) -> bool {
    let cell = board.get_cell_at(coordinate);

    match cell.space {
        Some(piece) => piece.color != current_piece_color,
        None => false,
    }
}

fn friendly_occupied(coordinate: Coordinate, current_piece_color: Color, board: &Board) -> bool {
    let cell = board.get_cell_at(coordinate);

    match cell.space {
        Some(piece) => piece.color == current_piece_color,
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
