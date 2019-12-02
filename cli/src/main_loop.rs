use std::io;

extern crate model;

use model::board::row_index_to_letter;
use model::board::Board;

pub fn execute_main_loop() {
    println!("Welcome to chess in Rust!");
    let mut guess = String::new();

    let board = model::get_board();

    println!("Current board state:");
    print_board_state(board);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your input: {}", guess);
}

fn print_board_state(board: Board) {
    for (k, v) in board.state.iter() {
        println!(
            "\n{:?}{:?}: {:?} {:?}",
            row_index_to_letter(k.0),
            k.1 + 1,
            v.color,
            v.classification
        );
    }
}
