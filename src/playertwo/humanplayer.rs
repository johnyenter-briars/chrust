use super::chessplayer::ChessPlayer;
use crate::board::Board;

#[derive(Debug)]
pub struct HumanPlayer {
    pub name: String
}

impl ChessPlayer for HumanPlayer {
    fn new(name: &str) -> Self {
        HumanPlayer{name: name.to_string()}    
    }
    
    fn take_action(board: Board) {
        println!("hello there from human!");
    }
}