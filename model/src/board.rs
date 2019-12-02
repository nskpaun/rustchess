use std::collections::HashMap;

use crate::piece::Piece;
use std::string::String;

pub struct Board {
    pub size: (u32, u32),
    pub state: HashMap<(u32, u32), Piece>,
}

pub fn row_index_to_letter(row_index: u32) -> String {
    let rows: Vec<char> = "ABCDEFGH".chars().collect();
    return rows[row_index as usize].to_string();
}

pub fn letter_to_row_index(row_letter: char) -> u32 {
    let rows: Vec<char> = "ABCDEFGH".chars().collect();
    let res = rows.iter().position(|&letter| letter == row_letter);
    return match res {
        Some(index) => index as u32,
        None => 9999,
    };
}
