use super::chessplayer::ChessPlayer;
use crate::board::{Board, cell::color::Color};

#[derive(Debug)]
pub struct AIPlayer {
    pub color: Color,
    pub name: String
}

impl ChessPlayer for AIPlayer {
    fn new(name: &str, color: Color) -> Self {
        AIPlayer{name: name.to_string(), color}    
    }
    
    fn take_action(board: Board) {
        println!("hello there from AI!");
    }
}
