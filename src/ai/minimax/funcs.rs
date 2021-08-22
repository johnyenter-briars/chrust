use core::panic;
use std::cmp;
use std::error::Error;

use crate::{
    board::{
        self,
        cell::color::{self, Color},
    },
    chessmove::piecemove::PieceMove,
};

use super::boardstate::BoardState;

pub fn max_value(board_state: BoardState, depth: i32) -> i32 {
    if depth == 0 {
        return board_state.get_state_eval();
    }

    let good_actions: Vec<BoardState> = Vec::new();

    let mut value = -1000000;

    let moves = match board_state.board.get_all_possible_moves(1, Color::Black) {
        Ok(mvs) => mvs,
        Err(e) => panic!("idk: {:?}", e),
    };

    for action in moves {
        let possible_value = min_value(board_state.apply_action(action), depth - 1);
        value = cmp::max(value, possible_value);
    }

    value
}

pub fn min_value(board_state: BoardState, depth: i32) -> i32 {
    if depth == 0 {
        return board_state.get_state_eval();
    }

    let good_actions: Vec<BoardState> = Vec::new();

    let mut value = 1000000;

    let moves = match board_state.board.get_all_possible_moves(1, Color::White) {
        Ok(mvs) => mvs,
        Err(e) => panic!("idk: {:?}", e),
    };

    for action in moves {
        let possible_value = max_value(board_state.apply_action(action), depth - 1);
        value = cmp::min(value, possible_value);
    }

    value
}

pub fn minimax_decition_min<'a>(
    board_state: &'a BoardState,
    color: Color,
    max_depth: i32,
) -> Result<(Option<PieceMove<'a>>, i32), Box<dyn Error>> {
    let mut good_actions: Vec<_> = Vec::new();

    for action in board_state.board.get_all_possible_moves(1, color)? {
        good_actions.push((
            action,
            max_value(board_state.apply_action(action), max_depth),
        ));
    }

    let mut min_action = (Option::None, 1000000);

    for action in good_actions {
        if action.1 <= min_action.1 {
            min_action.0 = Option::Some(action.0);
            min_action.1 = action.1;
        }
    }

    Ok(min_action)
}

pub fn minimax_decition_max<'a>(
    board_state: &'a BoardState,
    color: Color,
    max_depth: i32,
) -> Result<(Option<PieceMove<'a>>, i32), Box<dyn Error>> {
    let mut good_actions: Vec<_> = Vec::new();

    for action in board_state.board.get_all_possible_moves(1, color)? {
        good_actions.push((
            action,
            min_value(board_state.apply_action(action), max_depth),
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
