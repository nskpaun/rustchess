use crate::chess_move::ChessMove;
use model::classification::Classification;

use model::board::Board;

pub fn validate_chess_move(chess_move: &ChessMove, board: &Board) -> Result<(), ChessMoveError> {
    validate_move_is_on_board(chess_move, board)?;
    validate_move_is_legal_for_piece(chess_move)?;
    return Ok(());
}

fn validate_move_is_legal_for_piece(chess_move: &ChessMove) -> Result<(), ChessMoveError> {
    return match chess_move.piece {
        Classification::PAWN => Ok(()),
        Classification::ROOK => Ok(()),
        Classification::KNIGHT => Ok(()),
        Classification::BISHOP => Ok(()),
        Classification::KING => Ok(()),
        Classification::QUEEN => Ok(()),
    };
}

fn validate_move_is_on_board(chess_move: &ChessMove, board: &Board) -> Result<(), ChessMoveError> {
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
