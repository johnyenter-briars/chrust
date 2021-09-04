use crate::{board::chessboard::Board, chessmove::piecemove::PieceMove};

use super::evaluate::evaluate;

pub struct BoardState {
    pub board: Board
}

impl BoardState {
    pub fn state_eval(&self) -> i32 {
        evaluate(self)
    }

    pub fn apply_action(&self, action: PieceMove) -> BoardState {
        BoardState{board:self.board.apply_action(&action)}
    }
}
