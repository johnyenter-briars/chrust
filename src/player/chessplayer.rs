use crate::board::{chessboard::Board, cell::color::Color};

pub trait ChessPlayer {
    fn take_action(&self, board: Board);
    fn get_name(&self) -> &str;
    fn get_color_abbr(&self) -> char;
}
