use super::color::Color;
use super::piecetype::PieceType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl ChessPiece {
    pub fn get_opposite_color(&self) -> Color {
        match self.color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn get_str(&self) -> &str {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => "\u{2659}",
            (PieceType::Pawn, Color::Black) => "\u{265F}",
            (PieceType::Rook, Color::White) => "\u{2656}",
            (PieceType::Rook, Color::Black) => "\u{265C}",
            (PieceType::Knight, Color::White) => "\u{2658}",
            (PieceType::Knight, Color::Black) => "\u{265E}",
            (PieceType::Bishop, Color::White) => "\u{2657}",
            (PieceType::Bishop, Color::Black) => "\u{265D}",
            (PieceType::Queen, Color::White) => "\u{2654}",
            (PieceType::Queen, Color::Black) => "\u{265A}",
            (PieceType::King, Color::White) => "\u{2655}",
            (PieceType::King, Color::Black) => "\u{265B}",
        }
    }
}
