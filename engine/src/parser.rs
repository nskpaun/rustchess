extern crate strum;

use crate::chess_move;

use chess_move::ChessMove;
use model::board::letter_to_row_index;
use model::board::Board;
use model::classification::Classification;
use model::color::Color;
use std::string::String;
use strum::ParseError;

use std::str::FromStr;

pub fn parse_move(
    instruction: String,
    board: &Board,
    color: &Color,
) -> Result<ChessMove, ChessParseError> {
    let instruction_parts: Vec<&str> = instruction.split_whitespace().collect();

    let piece = Classification::from_str(instruction_parts[0])?;

    let origin = find_piece(color.clone(), piece.clone(), &board)?.clone();

    let column_chars: Vec<char> = instruction_parts[1].chars().collect();
    let column = letter_to_row_index(column_chars[0]);
    let row = instruction_parts[2].parse::<u32>().unwrap() - 1;

    return Ok(ChessMove {
        piece: piece,
        destination: (column, row),
        origin: origin,
    });
}

fn find_piece(
    color: Color,
    classification: Classification,
    board: &Board,
) -> Result<(&(u32, u32)), ChessParseError> {
    for (k, v) in board.state.iter() {
        if v.classification == classification && v.color == color {
            return Ok(k);
        }
    }
    return Err(ChessParseError {
        details: String::from("Could not find piece on board"),
    });
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
