use crate::player::{aiplayer::AIPlayer, humanplayer::HumanPlayer};
use super::piecemove::*;

pub struct HumanAction<'a> {
    piece_move: PieceMove<'a>,
    player: HumanPlayer,
}

pub struct AiAction<'a> {
    piece_move: PieceMove<'a>,
    player: AIPlayer,
}