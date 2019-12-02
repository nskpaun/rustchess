pub mod board;
mod piece;
mod classification;
mod color;

use board::Board as Board;
use piece::Piece as Piece;
use color::Color as Color;
use classification::Classification as Classification;

use std::collections::HashMap;

pub fn get_board() -> Board {
    let white_king = Piece {
        classification: Classification::KING,
        color: Color::WHITE
    };
    
    let black_king = Piece {
        classification: Classification::KING,
        color: Color::BLACK
    };

    let mut state: HashMap<(std::string::String, u32), Piece> = HashMap::new();

    state.insert((String::from("D"), 1), white_king);
    state.insert((String::from("D"), 8), black_king);

    let board = Board {
        size: 8,
        state: state,
    };
    return board;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
