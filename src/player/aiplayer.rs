use crate::board::cell::color::Color;

use super::chessplayer::ChessPlayer;

#[derive(Debug)]
pub struct AIPlayer {
    pub color: Color,
    pub name: String,
}

impl ChessPlayer for AIPlayer {
    fn name(&self) -> &str {
        &self.name
    }
    fn color_abbr(&self) -> char {
        self.color.color_abbr()
    }
}
