use crate::board::coordinate::Coordinate;

pub struct Decision {
	pub piece: &ChessPiece,
	pub from: Coordinate,
	pub to: Coordinate
}
