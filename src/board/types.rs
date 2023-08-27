#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PieceType {
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MoveType {
    EnPassant = 1,
    Castle = 2,
    Promotion = 3,
    CaptureOrMove = 4,
}