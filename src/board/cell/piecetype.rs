use std::{collections::HashSet, thread::current};

use crate::{board::{chessboard::Board, coordinate::Coordinate}, chessmove::piecemove::Move};

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

impl PieceType {
    pub fn available_moves(&self, target_piece: &ChessPiece, current_position: Coordinate, board: &Board, turn_num: i32) -> Vec<Coordinate> {
        let valid_moves: Vec<Coordinate> = match *self {
            PieceType::Pawn => { 
                //pawns can only move one or two spaces (depending on turn num)
                let possible_coordinates = if turn_num == 1 {
                    vec![current_position.up_by(1), current_position.up_by(2)]
                } else { 
                    vec![current_position.up_by(1)]
                };
                
                //only include the ones where the enemy is NOT in front of
                let mut possible_coordinates: Vec<Coordinate> = possible_coordinates.into_iter()
                                .filter(|coord| 
                                    ! enemy_occupied(coord.to_owned(), target_piece.color, board))
                                .map(|coord| coord.to_owned())
                                .collect();

                //Pawns can move diagonally if there's an enemy there
                let possible_enemy_positions: Vec<Coordinate> = vec![current_position.diagonal_up_right_by(1), current_position.diagonal_up_left_by(1)].into_iter()
                                .filter(|coord| coord.is_valid()) //filter out the ones that arent valid (i.e, diagonal left might be off the map)
                                .collect();
                
                for coord in possible_enemy_positions {
                    if enemy_occupied(coord, target_piece.color, board) { possible_coordinates.push(coord) };
                }

                possible_coordinates.into_iter().filter(|coord| coord.is_valid()).collect()
            },
            PieceType::Rook => todo!(),
            PieceType::Knight => todo!(),
            PieceType::Bishop => todo!(),
            PieceType::Queen => todo!(),
            PieceType::King => todo!(),
        };

        valid_moves
    }
}

fn filter_out_occupired_spaces(possible_coordinates: Vec<Coordinate>, board: &Board) -> Vec<Coordinate> {
    let empty_spaces_on_board = board.get_empty_spaces();

    //should do something cute with intersection here - but my first try was 'cant do intersection on non primitative types' so oh well
    possible_coordinates.into_iter() //need to into_iter in order to coerce rust to NOT iterate over references
            .filter(|coord| 
                empty_spaces_on_board.iter().any(|board_coord| 
                    board_coord.x == coord.x && board_coord.y == coord.y))
            .collect()
}

fn enemy_occupied(coordinate: Coordinate, color: Color, board: &Board) -> bool {
    let cell = board.get_cell_at(coordinate);

    match cell.space {
        Some(piece) => piece.color != color,
        None => false
    }
}