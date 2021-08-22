use crate::{board::{chessboard::Board, coordinate::Coordinate}, player::aiplayer::AIPlayer};
use crate::board::cell::color::Color;

use super::decision::Decision;

#[derive(Debug)]
pub struct DecisionMaker {

}

impl DecisionMaker {
	pub fn get_next_move<'a>(turn_num: i32, board: &'a Board, ai_player: &AIPlayer) -> Decision<'a>{
		Decision { piece: board.get_pieces_of_color(Color::Black).get(0).unwrap(), from: Coordinate::new('a', 2), to: Coordinate::new('a', 2)}
	}
}