use crate::board::{chessboard::Board, cell::color::Color};

pub trait ChessPlayer {
    fn take_action(&self, board: Board);
    fn name(&self) -> &str;
    fn color_abbr(&self) -> char;
}
