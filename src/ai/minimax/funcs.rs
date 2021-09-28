use core::panic;
use std::cmp;
use std::error::Error;
use std::time::Instant;

use crate::{
    ai::minimax::evaluate::evaluate,
    board::{cell::color::Color, chessboard::Board},
    chessmove::piecemove::PieceMove,
};

fn max_value(board: &Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let mut best_value = -1000000;

    let moves = match board.all_possible_moves(Color::Black) {
        Ok(mvs) => mvs,
        Err(e) => panic!(
            "Unable to get the possible moves from current board state: {:?}",
            e
        ),
    };

    for action in moves {
        best_value = cmp::max(
            best_value,
            min_value(&board.apply_action(&action), depth - 1, alpha, beta),
        );
        alpha = cmp::max(alpha, best_value);
        if beta <= alpha {
            return best_value;
        }
    }

    best_value
}

fn min_value(board: &Board, depth: i32, alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let mut best_value = 1000000;

    let moves = match board.all_possible_moves(Color::White) {
        Ok(mvs) => mvs,
        Err(e) => panic!(
            "Unable to get the possible moves from current board state: {:?}",
            e
        ),
    };

    for action in moves {
        best_value = cmp::min(
            best_value,
            max_value(&board.apply_action(&action), depth - 1, alpha, beta),
        );
        beta = cmp::min(beta, best_value);
        if beta <= alpha {
            return best_value;
        }
    }

    best_value
}

// fn minimax_decision_min<'a>(
//     board: &'a Board,
//     color: Color,
//     max_depth: i32,
// ) -> Result<(PieceMove<'a>, i32), Box<dyn Error>> {
//     let mut good_actions: Vec<_> = Vec::new();

//     for action in board.all_possible_moves(color)? {
//         good_actions.push((
//             action,
//             max_value(&board.apply_action(&action), max_depth),
//         ));
//     }

//     let min_action = good_actions.into_iter().min_by(|x, y| x.1.cmp(&y.1));

//     match min_action     {
//         Some(action) => Ok(action),
//         None => Err(Box::from("Minimax was unable to find a min decision!")),
//     }
// }

fn minimax_decision_max<'a>(
    board: &'a Board,
    color: Color,
    max_depth: i32,
    alpha: i32,
    beta: i32,
) -> Result<(PieceMove<'a>, i32), Box<dyn Error>> {
    let mut good_actions = Vec::new();

    for action in board.all_possible_moves(color)? {
        good_actions.push((
            action,
            min_value(&board.apply_action(&action), max_depth, alpha, beta),
        ));
    }

    let max_action = good_actions.into_iter().max_by(|x, y| x.1.cmp(&y.1));

    match max_action {
        Some(action) => Ok(action),
        None => Err(Box::from("Minimax was unable to find a max decision!")),
    }
}

pub fn max_decision<'a>(
    board: &'a Board,
    color: Color,
    max_depth: i32,
) -> Result<PieceMove<'a>, Box<dyn std::error::Error>> {
    let start = Instant::now();
    let (max_decision, _) = minimax_decision_max(board, color, max_depth, -1000000, 1000000)?;
    let duration = start.elapsed();
    println!("Time to determine move: {:?}", duration);
    Ok(max_decision)
}
