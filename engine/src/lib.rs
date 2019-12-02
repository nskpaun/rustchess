extern crate model;
mod chess_move;
mod parser;

use model::board::Board;
use model::classification::Classification;
use model::color::Color;
use std::string::String;

pub fn execute_move(
    color: &Color,
    instruction: String,
    mut board: Board,
) -> Result<Board, ChessMoveError> {
    let chess_move = match parser::parse_move(instruction) {
        Ok(chess_move_res) => Ok(chess_move_res),
        Err(err) => Err(ChessMoveError {
            details: err.details,
            board: board.clone(),
        }),
    }?;
    let origin = find_piece(color.clone(), chess_move.piece, &board)?.clone();
    let destination = chess_move.destination;

    if destination.0 > board.size.0 - 1 || destination.1 > board.size.1 - 1 {
        return Err(ChessMoveError {
            details: String::from("Illegal index for move"),
            board: board,
        });
    }
    let piece = match board.get(&origin) {
        Some(piece_res) => Ok(piece_res),
        None => Err(ChessMoveError {
            details: String::from("Did not find Piece"),
            board: board.clone(),
        }),
    }?
    .clone();

    board.remove(&origin);
    board.insert(destination, piece);
    return Ok(board);
}

fn find_piece(
    color: Color,
    classification: Classification,
    board: &Board,
) -> Result<(&(u32, u32)), ChessMoveError> {
    for (k, v) in board.state.iter() {
        if v.classification == classification && v.color == color {
            return Ok(k);
        }
    }
    return Err(ChessMoveError {
        details: String::from("Could not find piece on board"),
        board: board.clone(),
    });
}

pub struct ChessMoveError {
    pub details: String,
    pub board: Board,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
