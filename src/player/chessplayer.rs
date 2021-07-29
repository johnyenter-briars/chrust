use crate::board::Board;

pub trait ChessPlayer {
    fn new(name: &str) -> Self; 
    fn take_action(board: Board);
}

// pub struct HumanPlayer