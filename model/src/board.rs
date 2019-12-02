use std::collections::HashMap;

use crate::piece::Piece;

pub struct Board {
    pub size: u32,
    pub state: HashMap<(std::string::String, u32), Piece>,
}