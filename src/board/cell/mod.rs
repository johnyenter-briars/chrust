pub mod chesspiece;
use chesspiece::ChessPiece;

#[derive(Debug, Clone, Copy)]
pub struct Cell{
    pub space: Option<ChessPiece>
}
