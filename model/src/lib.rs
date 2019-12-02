pub mod board;
mod classification;
mod color;
mod piece;

use board::letter_to_row_index;
use board::Board;
use classification::Classification;
use color::Color;
use piece::Piece;

use std::collections::HashMap;

pub fn get_board() -> Board {
    let mut state: HashMap<(u32, u32), Piece> = HashMap::new();

    state = add_piece(state, 'A', 2, Classification::PAWN, Color::WHITE);
    state = add_piece(state, 'B', 2, Classification::PAWN, Color::WHITE);
    state = add_piece(state, 'C', 2, Classification::PAWN, Color::WHITE);
    state = add_piece(state, 'D', 2, Classification::PAWN, Color::WHITE);
    state = add_piece(state, 'E', 2, Classification::PAWN, Color::WHITE);
    state = add_piece(state, 'F', 2, Classification::PAWN, Color::WHITE);
    state = add_piece(state, 'G', 2, Classification::PAWN, Color::WHITE);
    state = add_piece(state, 'H', 2, Classification::PAWN, Color::WHITE);

    state = add_piece(state, 'A', 1, Classification::ROOK, Color::WHITE);
    state = add_piece(state, 'B', 1, Classification::KNIGHT, Color::WHITE);
    state = add_piece(state, 'C', 1, Classification::BISHOP, Color::WHITE);
    state = add_piece(state, 'D', 1, Classification::KING, Color::WHITE);
    state = add_piece(state, 'E', 1, Classification::QUEEN, Color::WHITE);
    state = add_piece(state, 'F', 1, Classification::BISHOP, Color::WHITE);
    state = add_piece(state, 'G', 1, Classification::KNIGHT, Color::WHITE);
    state = add_piece(state, 'H', 1, Classification::ROOK, Color::WHITE);

    state = add_piece(state, 'A', 7, Classification::PAWN, Color::BLACK);
    state = add_piece(state, 'B', 7, Classification::PAWN, Color::BLACK);
    state = add_piece(state, 'C', 7, Classification::PAWN, Color::BLACK);
    state = add_piece(state, 'D', 7, Classification::PAWN, Color::BLACK);
    state = add_piece(state, 'E', 7, Classification::PAWN, Color::BLACK);
    state = add_piece(state, 'F', 7, Classification::PAWN, Color::BLACK);
    state = add_piece(state, 'G', 7, Classification::PAWN, Color::BLACK);
    state = add_piece(state, 'H', 7, Classification::PAWN, Color::BLACK);

    state = add_piece(state, 'A', 8, Classification::ROOK, Color::BLACK);
    state = add_piece(state, 'B', 8, Classification::KNIGHT, Color::BLACK);
    state = add_piece(state, 'C', 8, Classification::BISHOP, Color::BLACK);
    state = add_piece(state, 'D', 8, Classification::QUEEN, Color::BLACK);
    state = add_piece(state, 'E', 8, Classification::KING, Color::BLACK);
    state = add_piece(state, 'F', 8, Classification::BISHOP, Color::BLACK);
    state = add_piece(state, 'G', 8, Classification::KNIGHT, Color::BLACK);
    state = add_piece(state, 'H', 8, Classification::ROOK, Color::BLACK);

    let board = Board {
        size: (8, 8),
        state: state,
    };
    return board;
}

fn add_piece(
    mut state: HashMap<(u32, u32), Piece>,
    column: char,
    row: u32,
    classification: Classification,
    color: Color,
) -> HashMap<(u32, u32), Piece> {
    state.insert(
        (letter_to_row_index(column), row - 1),
        Piece {
            classification: classification,
            color: color,
        },
    );
    return state;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
