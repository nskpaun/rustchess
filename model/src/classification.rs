extern crate strum;
extern crate strum_macros;
#[macro_use]

use strum_macros::{Display};

#[derive(Display, Debug)]
pub enum Classification {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING,
}