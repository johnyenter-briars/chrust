use crate::board::{cell::chesspiece::ChessPiece, coordinate::Coordinate};

pub struct Decision<'a> {
	pub piece: &'a ChessPiece,
	pub from: Coordinate,
	pub to: Coordinate
}