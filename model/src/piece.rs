use crate::classification::Classification;
use crate::color::Color;

#[derive(Clone)]
pub struct Piece {
    pub classification: Classification,
    pub color: Color,
}