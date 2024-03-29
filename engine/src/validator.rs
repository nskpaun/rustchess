use crate::chess_move::ChessMove;
use model::classification::Classification;
use model::color::Color;
use std::vec::Vec;

use model::board::Board;

pub fn validate_chess_move(
    chess_move: &ChessMove,
    board: &Board,
    color: &Color,
) -> Result<(), ChessMoveError> {
    validate_move_is_on_board(chess_move, board)?;
    validate_move_is_legal_for_piece(chess_move, board, color)?;
    validate_path_is_clear(chess_move, board, color)?;
    return Ok(());
}

fn validate_path_is_clear(
    chess_move: &ChessMove,
    board: &Board,
    color: &Color,
) -> Result<(), ChessMoveError> {
    let is_destination_occupied = match board.get(&chess_move.destination) {
        Some(piece_res) => piece_res.color == *color,
        None => false,
    };

    if is_destination_occupied {
        return Err(ChessMoveError {
            details: String::from("Destination square is occupied"),
            board: board.clone(),
        });
    }

    match chess_move.piece {
        Classification::KING | Classification::KNIGHT => Ok(()),
        Classification::ROOK => {
            return validate_horizontal_or_vertical_path(
                chess_move,
                board,
                String::from("Rook move illegal, blocked by piece"),
            );
        }
        Classification::BISHOP => {
            return validate_diagonal_path(
                chess_move,
                board,
                String::from("Bishop move illegal, blocked by piece"),
            );
        }
        Classification::QUEEN => {
            let column_diff = (chess_move.origin.0 - chess_move.destination.0).abs();
            let row_diff = (chess_move.origin.1 - chess_move.destination.1).abs();
            if column_diff == row_diff {
                validate_diagonal_path(
                    chess_move,
                    board,
                    String::from("Queen move illegal, blocked by piece"),
                )?;
            } else {
                validate_horizontal_or_vertical_path(
                    chess_move,
                    board,
                    String::from("Queen move illegal, blocked by piece"),
                )?;
            }

            return Ok(());
        }
        Classification::PAWN => {
            let row_diff = chess_move.destination.1 - chess_move.origin.1;
            let is_middle_occupied =
                match board.get(&(chess_move.origin.0, chess_move.origin.1 + row_diff / 2)) {
                    Some(piece_res) => piece_res.color == *color,
                    None => false,
                };
            if row_diff.abs() == 2 && is_middle_occupied {
                return Err(ChessMoveError {
                    details: String::from("Pawn move illegal, blocked by piece"),
                    board: board.clone(),
                });
            }

            return Ok(());
        }
    }
}

fn validate_diagonal_path(
    chess_move: &ChessMove,
    board: &Board,
    error_message: String,
) -> Result<(), ChessMoveError> {
    let rows: Vec<i32> = (chess_move.origin.1..chess_move.destination.1).collect();
    let columns: Vec<i32> = (chess_move.origin.0..chess_move.destination.0).collect();
    for i in 0..rows.len() {
        let current_check = (columns[i], rows[i]);
        if current_check == chess_move.origin || current_check == chess_move.destination {
            continue;
        }
        let is_current_check_occupied = match board.get(&current_check) {
            Some(_) => true,
            None => false,
        };
        if is_current_check_occupied {
            return Err(ChessMoveError {
                details: error_message,
                board: board.clone(),
            });
        }
    }
    return Ok(());
}

fn validate_horizontal_or_vertical_path(
    chess_move: &ChessMove,
    board: &Board,
    error_message: String,
) -> Result<(), ChessMoveError> {
    let is_vertical_move = chess_move.origin.0 == chess_move.destination.0;
    if is_vertical_move {
        for row_check in chess_move.origin.1..chess_move.destination.1 {
            let current_check = (chess_move.origin.0, row_check);
            if current_check == chess_move.origin || current_check == chess_move.destination {
                continue;
            }
            let is_current_check_occupied = match board.get(&current_check) {
                Some(_) => true,
                None => false,
            };
            if is_current_check_occupied {
                return Err(ChessMoveError {
                    details: error_message,
                    board: board.clone(),
                });
            }
        }
    } else {
        for column_check in chess_move.origin.0..chess_move.destination.0 {
            let current_check = (chess_move.origin.1, column_check);
            if current_check == chess_move.origin || current_check == chess_move.destination {
                continue;
            }
            let is_current_check_occupied = match board.get(&current_check) {
                Some(_) => true,
                None => false,
            };
            if is_current_check_occupied {
                return Err(ChessMoveError {
                    details: String::from("Rook move illegal, blocked by piece"),
                    board: board.clone(),
                });
            }
        }
    }

    return Ok(());
}

fn validate_move_is_legal_for_piece(
    chess_move: &ChessMove,
    board: &Board,
    color: &Color,
) -> Result<(), ChessMoveError> {
    match chess_move.piece {
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
        Classification::BISHOP => {
            let column_diff = (chess_move.origin.0 - chess_move.destination.0).abs();
            let row_diff = (chess_move.origin.1 - chess_move.destination.1).abs();
            if column_diff == row_diff && row_diff > 0 && column_diff > 0 {
                return Ok(());
            }
            return Err(ChessMoveError {
                details: String::from("Illegal bishop move"),
                board: board.clone(),
            });
        }
        Classification::KING => {
            let column_diff = (chess_move.origin.0 - chess_move.destination.0).abs();
            let row_diff = (chess_move.origin.1 - chess_move.destination.1).abs();
            if column_diff > 1 || row_diff > 1 {
                return Err(ChessMoveError {
                    details: String::from("Illegal king move"),
                    board: board.clone(),
                });
            }
            if column_diff == row_diff && row_diff > 0 && column_diff > 0 {
                return Ok(());
            }

            let is_valid_horizonatal_move = chess_move.origin.0 != chess_move.destination.0
                && chess_move.origin.1 == chess_move.destination.1;
            let is_valid_vertical_move = chess_move.origin.0 == chess_move.destination.0
                && chess_move.origin.1 != chess_move.destination.1;

            if is_valid_horizonatal_move || is_valid_vertical_move {
                return Ok(());
            }

            return Err(ChessMoveError {
                details: String::from("Illegal king move"),
                board: board.clone(),
            });
        }
        Classification::QUEEN => {
            let column_diff = (chess_move.origin.0 - chess_move.destination.0).abs();
            let row_diff = (chess_move.origin.1 - chess_move.destination.1).abs();
            if column_diff == row_diff && row_diff > 0 && column_diff > 0 {
                return Ok(());
            }

            let is_valid_horizonatal_move = chess_move.origin.0 != chess_move.destination.0
                && chess_move.origin.1 == chess_move.destination.1;
            let is_valid_vertical_move = chess_move.origin.0 == chess_move.destination.0
                && chess_move.origin.1 != chess_move.destination.1;

            if is_valid_horizonatal_move || is_valid_vertical_move {
                return Ok(());
            }

            return Err(ChessMoveError {
                details: String::from("Illegal queen move"),
                board: board.clone(),
            });
        }
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
