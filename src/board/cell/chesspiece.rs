use super::color::Color;
use super::piecetype::PieceType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl ChessPiece {
    pub fn str(&self) -> &str {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => "\u{2659}",
            (PieceType::Pawn, Color::Black) => "\u{265F}",
            (PieceType::Rook, Color::White) => "\u{2656}",
            (PieceType::Rook, Color::Black) => "\u{265C}",
            (PieceType::Knight, Color::White) => "\u{2658}",
            (PieceType::Knight, Color::Black) => "\u{265E}",
            (PieceType::Bishop, Color::White) => "\u{2657}",
            (PieceType::Bishop, Color::Black) => "\u{265D}",
            (PieceType::Queen, Color::White) => "\u{2655}",
            (PieceType::Queen, Color::Black) => "\u{265B}",
            (PieceType::King, Color::White) => "\u{2654}",
            (PieceType::King, Color::Black) => "\u{265A}",
        }
    }

    pub fn fen_code(&self) -> &str {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => "P",
            (PieceType::Pawn, Color::Black) => "p",
            (PieceType::Rook, Color::White) => "R",
            (PieceType::Rook, Color::Black) => "r",
            (PieceType::Knight, Color::White) => "N",
            (PieceType::Knight, Color::Black) => "n",
            (PieceType::Bishop, Color::White) => "B",
            (PieceType::Bishop, Color::Black) => "b",
            (PieceType::Queen, Color::White) => "Q",
            (PieceType::Queen, Color::Black) => "q",
            (PieceType::King, Color::White) => "K",
            (PieceType::King, Color::Black) => "k",
        }
    }
}
