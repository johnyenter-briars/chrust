pub mod chesspiece;
use chesspiece::ChessPiece;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell{
    pub space: Option<ChessPiece>
}
