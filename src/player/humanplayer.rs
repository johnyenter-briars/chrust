use super::chessplayer::ChessPlayer;
use crate::board::cell::color::Color;

#[derive(Debug)]
pub struct HumanPlayer {
    pub name: String,
    pub color: Color,
}

impl ChessPlayer for HumanPlayer {
    fn name(&self) -> &str {
        &self.name
    }
    fn color_abbr(&self) -> char {
        self.color.color_abbr()
    }
}
