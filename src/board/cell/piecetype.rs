use crate::board::chessboard::Board;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PieceType {
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
}

impl PieceType {
    fn available_moves(&self, board: Board, turn_num: i32) -> Vec<&str> {
        let valid_move = match *self {
            PieceType::Pawn => { //pawns can only move one or two spaces 
                
                


            },
            PieceType::Rook => todo!(),
            PieceType::Knight => todo!(),
            PieceType::Bishop => todo!(),
            PieceType::Queen => todo!(),
            PieceType::King => todo!(),
        };

        vec!["test"]
    }
}
