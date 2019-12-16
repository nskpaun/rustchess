extern crate strum;

use crate::chess_move;
use crate::validator::validate_chess_move;

use chess_move::ChessMove;
use model::board::letter_to_row_index;
use model::board::Board;
use model::classification::Classification;
use model::color::Color;
use std::string::String;
use std::vec::Vec;
use strum::ParseError;

use std::str::FromStr;

pub fn parse_move(
    instruction: String,
    board: &Board,
    color: &Color,
) -> Result<ChessMove, ChessParseError> {
    let instruction_parts: Vec<&str> = instruction.split_whitespace().collect();

    let piece;
    let column;
    let origin_column;
    if instruction_parts.len() == 2 {
        piece = Classification::PAWN;
        let column_chars: Vec<char> = instruction_parts[0].chars().collect();
        column = letter_to_row_index(column_chars[0]);
        origin_column = column;
    } else if instruction_parts.len() == 4 {
        piece = Classification::from_str(instruction_parts[0])?;
        let origin_column_chars: Vec<char> = instruction_parts[1].chars().collect();
        origin_column = letter_to_row_index(origin_column_chars[0]);
        let column_chars: Vec<char> = instruction_parts[2].chars().collect();
        column = letter_to_row_index(column_chars[0]);
    } else {
        piece = Classification::from_str(instruction_parts[0])?;
        let column_chars: Vec<char> = instruction_parts[1].chars().collect();
        column = letter_to_row_index(column_chars[0]);
        origin_column = 9999;
    }

    let possible_origins = find_piece(color.clone(), piece.clone(), &board, origin_column)?.clone();

    let mut error_details = String::from("Could not find piece on board");

    for origin in possible_origins.iter() {
        let row = instruction_parts[instruction_parts.len() - 1]
            .parse::<i32>()
            .unwrap()
            - 1;
        let chess_move = ChessMove {
            piece: piece.clone(),
            destination: (column, row),
            origin: **origin,
        };
        match validate_chess_move(&chess_move, board, color) {
            Ok(_) => return Ok(chess_move),
            Err(err) => {
                error_details = err.details;
                continue;
            }
        }
    }

    return Err(ChessParseError {
        details: error_details,
    });
}

fn find_piece(
    color: Color,
    classification: Classification,
    board: &Board,
    origin_column: i32,
) -> Result<(Vec<&(i32, i32)>), ChessParseError> {
    let mut pieces = Vec::<&(i32, i32)>::with_capacity(8);
    for (k, v) in board.state.iter() {
        let is_correct_column = origin_column > board.size.0 || origin_column == k.0;
        if v.classification == classification && v.color == color && is_correct_column {
            pieces.push(k);
        }
    }
    if pieces.is_empty() {
        return Err(ChessParseError {
            details: String::from("Could not find piece on board"),
        });
    }
    return Ok(pieces);
}

#[derive(Debug)]
pub struct ChessParseError {
    pub details: String,
}

impl ChessParseError {
    fn new(msg: &str) -> ChessParseError {
        ChessParseError {
            details: msg.to_string(),
        }
    }
}

impl From<ParseError> for ChessParseError {
    fn from(_err: ParseError) -> Self {
        ChessParseError::new("error parsing piece")
    }
}
