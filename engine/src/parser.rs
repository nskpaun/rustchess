extern crate strum;

use crate::chess_move;

use chess_move::ChessMove;
use model::board::letter_to_row_index;
use model::classification::Classification;
use std::string::String;
use strum::ParseError;

use std::str::FromStr;

pub fn parse_move(instruction: String) -> Result<ChessMove, ChessParseError> {
    let instruction_parts: Vec<&str> = instruction.split_whitespace().collect();

    let piece = Classification::from_str(instruction_parts[0])?;

    let column_chars: Vec<char> = instruction_parts[1].chars().collect();
    let column = letter_to_row_index(column_chars[0]);
    let row = instruction_parts[2].parse::<u32>().unwrap() - 1;

    return Ok(ChessMove {
        piece: piece,
        destination: (column, row),
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
