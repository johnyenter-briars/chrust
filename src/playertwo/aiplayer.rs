use super::chessplayer::ChessPlayer;
use crate::board::Board;

#[derive(Debug)]
pub struct AIPlayer {
    pub name: String
}

impl ChessPlayer for AIPlayer {
    fn new(name: &str) -> Self {
        AIPlayer{name: name.to_string()}    
    }
    
    fn take_action(board: Board) {
        println!("hello there from AI!");
    }
}
