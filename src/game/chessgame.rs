use crate::ai::minimax::funcs::max_decision;
use crate::board::chessboard::ChessBoard;
use crate::board::color::Color;
use crate::board::coordinate::{Coordinate, ToCoord};
use crate::board::piece::piecetype::PieceType;
use crate::chessmove::piecemove::PieceMove;
use crate::ChrustResult;

extern crate rand;

use itertools::Itertools;

pub fn process_fen(
    fen: String,
    source: Coordinate,
    target: Coordinate,
    default_promotion_piece: PieceType,
    num_plies: u32,
) -> ChrustResult<String> {
    let mut board_before_white_action = ChessBoard::load_from_fen(&fen)?;
    board_before_white_action.default_promotion_piece = Some(default_promotion_piece);

    board_before_white_action.print_to_screen(&board_before_white_action.fen());

    let chess_piece = board_before_white_action.piece(source.x, source.y)?;

    let options = chess_piece.available_moves(&board_before_white_action);

    let binding = options
        .iter()
        .filter(|o| o.0.x == target.x && o.0.y == target.y)
        .map(|o| o.1)
        .collect_vec();

    let move_type = binding.first().expect("No move found for that input :(");

    let action = &PieceMove::new(chess_piece, source, target, Color::White, *move_type);

    let mut board_after_white_action = board_before_white_action.apply_action(action)?;

    board_after_white_action.print_to_screen(&board_after_white_action.fen());
    board_after_white_action.default_promotion_piece = None;

    let ai_move = max_decision(&board_after_white_action, Color::Black, num_plies)?;

    let board_after_black_action = board_after_white_action.apply_action(&ai_move)?;

    board_after_black_action.print_to_screen("ai move");

    Ok(board_after_black_action.fen())
}

pub fn is_valid(
    fen: String,
    current_location: String,
    possible_location: String,
) -> ChrustResult<bool> {
    let board = ChessBoard::load_from_fen(&fen)?;

    let current_position = current_location.to_coord();

    let piece = board.piece(current_position.x, current_position.y)?;
    let possible_moves = piece.available_moves(&board);

    let possible_position = possible_location.to_coord();

    let valid_move = possible_moves
        .iter()
        .any(|coord| coord.0.x == possible_position.x && coord.0.y == possible_position.y);

    Ok(valid_move)
}

pub fn valid_moves(fen: String, location: String) -> ChrustResult<Vec<Coordinate>> {
    let board = ChessBoard::load_from_fen(&fen)
        .unwrap_or_else(|_| panic!("Unable to load board from fen: {}", fen));

    if location.as_str().len() != 2 {
        return Err(Box::from("Invalid format for location string"));
    }

    let current_position = location.to_coord();

    let piece = board.piece(current_position.x, current_position.y)?;
    if piece.color() == Color::Black {
        return Ok(vec![]);
    }

    let coords = piece
        .available_moves(&board)
        .iter()
        .map(|c| c.0)
        .collect_vec();

    Ok(coords)
}
