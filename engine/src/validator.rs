use crate::chess_move::ChessMove;

use model::board::Board;

pub fn validate_chess_move(chess_move: &ChessMove, board: &Board) -> Result<(), ChessMoveError> {
    let destination = chess_move.destination;

    if destination.0 > board.size.0 - 1 || destination.1 > board.size.1 - 1 {
        return Err(ChessMoveError {
            details: String::from("Illegal index for move"),
            board: board.clone(),
        });
    }
    return Ok(());
}

pub struct ChessMoveError {
    pub details: String,
    pub board: Board,
}
