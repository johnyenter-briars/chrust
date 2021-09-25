use core::panic;
use std::cmp;
use std::error::Error;

use crate::{ai::minimax::evaluate::evaluate, board::{cell::color::Color, chessboard::Board}, chessmove::piecemove::PieceMove};

// use super::boardstate::BoardState;

fn max_value(board: &Board, depth: i32) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let mut value = -1000000;

    let moves = match board.all_possible_moves(Color::Black) {
        Ok(mvs) => mvs,
        Err(e) => panic!("Unable to get the possible moves from current board state: {:?}", e),
    };

    for action in moves {
        let possible_value = min_value(&board.apply_action(&action), depth - 1);
        value = cmp::max(value, possible_value);
    }

    value
}

fn min_value(board: &Board, depth: i32) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let mut value = 1000000;

    let moves = match board.all_possible_moves(Color::White) {
        Ok(mvs) => mvs,
        Err(e) => panic!("Unable to get the possible moves from current board state: {:?}", e),
    };

    for action in moves {
        let possible_value = max_value(&board.apply_action(&action), depth - 1);
        value = cmp::min(value, possible_value);
    }

    value
}

// fn minimax_decision_min<'a>(
//     board: &'a Board,
//     color: Color,
//     max_depth: i32,
// ) -> Result<(Option<PieceMove<'a>>, i32), Box<dyn Error>> {
//     let mut good_actions: Vec<_> = Vec::new();

//     for action in board.all_possible_moves(color)? {
//         good_actions.push((
//             action,
//             max_value(&board.apply_action(&action), max_depth),
//         ));
//     }

//     let mut min_action = (Option::None, 1000000);

//     for action in good_actions {
//         if action.1 <= min_action.1 {
//             min_action.0 = Option::Some(action.0);
//             min_action.1 = action.1;
//         }
//     }

//     Ok(min_action)
// }

fn minimax_decision_max<'a>(
    board: &'a Board,
    color: Color,
    max_depth: i32,
) -> Result<(Option<PieceMove<'a>>, i32), Box<dyn Error>> {
    let mut good_actions: Vec<_> = Vec::new();

    for action in board.all_possible_moves(color)? {
        good_actions.push((
            action,
            min_value(&board.apply_action(&action), max_depth),
        ));
    }

    let mut max_action = (Option::None, -1000000);

    for action in good_actions {
        if action.1 > max_action.1 {
            max_action.0 = Option::Some(action.0);
            max_action.1 = action.1;
        }
    }

    Ok(max_action)
}

pub fn max_decision<'a>(
    board: &'a Board,
    color: Color,
    max_depth: i32,
) -> Result<PieceMove<'a>, Box<dyn std::error::Error>> {
    let (max_decision, _) = minimax_decision_max(board, color, max_depth)?;

    match max_decision {
        Some(piece_move) => Ok(piece_move),
        None => Err(Box::from("tes"))
    }
}
