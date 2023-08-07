use super::{piece::{achesspiece::AChessPiece}, color::Color};


#[derive(Debug)]
pub struct Cell {
    pub space: Option<Box<dyn AChessPiece>>,
    pub color: Color,
    pub x: char,
    pub y: i32,
}

impl Cell {
    pub fn new(x: char, y: i32, color: Color, space: Option<Box<dyn AChessPiece>>) -> Self {
        Cell { x, y, color, space }
    }
}