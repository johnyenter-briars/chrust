pub trait ChessPlayer {
    fn name(&self) -> &str;
    fn color_abbr(&self) -> char;
}
