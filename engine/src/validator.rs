use crate::chess_move::ChessMove;
use model::classification::Classification;
use model::color::Color;

use model::board::Board;

pub fn validate_chess_move(
    chess_move: &ChessMove,
    board: &Board,
    color: &Color,
) -> Result<(), ChessMoveError> {
    validate_move_is_on_board(chess_move, board)?;
    validate_move_is_legal_for_piece(chess_move, board, color)?;
    return Ok(());
}

fn validate_move_is_legal_for_piece(
    chess_move: &ChessMove,
    board: &Board,
    color: &Color,
) -> Result<(), ChessMoveError> {
    return match chess_move.piece {
        Classification::PAWN => {
            // only allow moving 2 spaces if on 2nd or 7th rank
            let move_direction = match color {
                Color::WHITE => 1,
                Color::BLACK => -1,
            };
            let is_capturing = match board.get(&chess_move.destination) {
                Some(piece_res) => piece_res.color != *color,
                None => false,
            };

            // only allowed to move forward one square
            if chess_move.origin.0 == chess_move.destination.0
                && chess_move.destination.1 - chess_move.origin.1 == move_direction
                && !is_capturing
            {
                return Ok(());
            }

            // only allow moving 2 spaces if on 2nd or 7th rank
            let initial_rank = match color {
                Color::WHITE => 1,
                Color::BLACK => 6,
            };

            if chess_move.origin.1 == initial_rank
                && chess_move.origin.0 == chess_move.destination.0
                && chess_move.destination.1 - chess_move.origin.1 == 2 * move_direction
                && !is_capturing
            {
                return Ok(());
            }

            // allow capturing diagonally forward
            if is_capturing
                && chess_move.destination.1 - chess_move.origin.1 == move_direction
                && chess_move.destination.0 - chess_move.origin.0 == move_direction
            {
                return Ok(());
            }

            return Err(ChessMoveError {
                details: String::from("Illegal pawn move"),
                board: board.clone(),
            });
        }
        Classification::ROOK => {
            let is_valid_horizonatal_move = chess_move.origin.0 != chess_move.destination.0
                && chess_move.origin.1 == chess_move.destination.1;
            let is_valid_vertical_move = chess_move.origin.0 == chess_move.destination.0
                && chess_move.origin.1 != chess_move.destination.1;

            if is_valid_horizonatal_move || is_valid_vertical_move {
                return Ok(());
            }

            return Err(ChessMoveError {
                details: String::from("Illegal rook move"),
                board: board.clone(),
            });
        }
        Classification::KNIGHT => {
            let column_diff = (chess_move.origin.0 - chess_move.destination.0).abs();
            let row_diff = (chess_move.origin.1 - chess_move.destination.1).abs();
            if row_diff + column_diff == 3 && row_diff > 0 && column_diff > 0 {
                return Ok(());
            }
            return Err(ChessMoveError {
                details: String::from("Illegal knight move"),
                board: board.clone(),
            });
        }
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
