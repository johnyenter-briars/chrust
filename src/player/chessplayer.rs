use crate::board::{Board, cell::color::Color};

pub trait ChessPlayer {
    fn new(name: &str, color: Color) -> Self; 
    fn take_action(board: Board);
}

// pub struct HumanPlayer