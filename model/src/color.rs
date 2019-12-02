extern crate strum;
extern crate strum_macros;
#[macro_use]

use strum_macros::{Display};

#[derive(Display, Debug, Clone, PartialEq)]
pub enum Color {
    WHITE,
    BLACK
}