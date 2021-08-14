use crate::board::chessboard::Board;

pub struct BoardState {
    pub board: Board
}

impl BoardState {
    pub fn get_state_eval() -> i32 {
        1
    }
}
