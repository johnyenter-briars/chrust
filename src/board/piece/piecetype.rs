#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PieceType {
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
}

pub trait ToPieceType {
    fn to_piece_type(&self) -> Option<PieceType>;
}

impl ToPieceType for std::string::String {
    fn to_piece_type(&self) -> Option<PieceType> {
        // match self
        match self.as_str() {
            "rook" => Some(PieceType::Rook),
            "bishop" => Some(PieceType::Bishop),
            "queen" => Some(PieceType::Queen),
            "knight" => Some(PieceType::Knight),
            _ => None,
        }
    }
}
