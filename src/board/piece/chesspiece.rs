use crate::board::{chessboard::ChessBoard, color::Color, coordinate::Coordinate};

use super::achesspiece::AChessPiece;

// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// pub struct ChessPiece {
//     pub color: Color,
//     pub current_position: Coordinate,
// }

// impl AChessPiece for ChessPiece {
//     fn available_moves(
//         &self,
//         board: &Board,
//     ) -> Vec<Coordinate> {
//         vec![]
//     }
//     fn str(&self) -> &str {
//         todo!()
//     }
//     fn fen_code(&self) -> &str {
//         todo!()

//     }
//     fn get_piece_type(&self) -> crate::board::piecetype::PieceType {
//         todo!();
//     }
// }
