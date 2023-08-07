use super::{piece::{achesspiece::AChessPiece}, color::Color};


pub struct Cell {
    pub space: Option<Box<dyn AChessPiece>>,
    pub color: Color,
    pub x: char,
    pub y: i32,
}