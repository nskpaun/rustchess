use std::collections::HashMap;

use crate::piece::Piece;
use std::string::String;

#[derive(Clone)]
pub struct Board {
    pub size: (i32, i32),
    pub state: HashMap<(i32, i32), Piece>,
}

impl Board {
    pub fn remove(&mut self, origin: &(i32, i32)) {
        self.state.remove(origin);
    }

    pub fn insert(&mut self, destination: (i32, i32), piece: Piece) {
        self.state.insert(destination, piece);
    }

    pub fn get(&self, origin: &(i32, i32)) -> std::option::Option<&Piece> {
        return self.state.get(origin);
    }
}

pub fn row_index_to_letter(row_index: i32) -> String {
    let rows: Vec<char> = "ABCDEFGH".chars().collect();
    return rows[row_index as usize].to_string();
}

pub fn letter_to_row_index(row_letter: char) -> i32 {
    let rows: Vec<char> = "ABCDEFGH".chars().collect();
    let res = rows.iter().position(|&letter| letter == row_letter);
    return match res {
        Some(index) => index as i32,
        None => 9999,
    };
}
