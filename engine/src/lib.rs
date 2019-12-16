extern crate model;
mod chess_move;
mod parser;
pub mod validator;

use model::board::Board;
use model::color::Color;
use std::string::String;
use validator::ChessMoveError;
use validator::validate_chess_move;

pub fn execute_move(
    color: &Color,
    instruction: String,
    mut board: Board,
) -> Result<Board, ChessMoveError> {
    let chess_move = match parser::parse_move(instruction, &board, color) {
        Ok(chess_move_res) => Ok(chess_move_res),
        Err(err) => Err(ChessMoveError {
            details: err.details,
            board: board.clone(),
        }),
    }?;
    let destination = chess_move.destination;

    validate_chess_move(&chess_move, &board, color)?;

    let piece = match board.get(&chess_move.origin) {
        Some(piece_res) => Ok(piece_res),
        None => Err(ChessMoveError {
            details: String::from("Did not find Piece"),
            board: board.clone(),
        }),
    }?
    .clone();

    board.remove(&chess_move.origin);
    board.insert(destination, piece);
    return Ok(board);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
