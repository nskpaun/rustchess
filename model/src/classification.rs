extern crate strum;
extern crate strum_macros;
#[macro_use]

use strum_macros::{Display, EnumString};

#[derive(Display, EnumString, Debug, Clone, PartialEq)]
pub enum Classification {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING,
}