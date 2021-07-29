use super::chessplayer::ChessPlayer;
use crate::board::{Board, cell::color::Color};

#[derive(Debug)]
pub struct HumanPlayer {
    pub name: String,
    pub color: Color,
}

impl ChessPlayer for HumanPlayer {
    fn new(name: &str, color: Color) -> Self {
        HumanPlayer{name: name.to_string(), color}    
    }
    
    fn take_action(board: Board) {
        println!("hello there from human!");
    }
}