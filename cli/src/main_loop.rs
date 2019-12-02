use std::io;

extern crate engine;
extern crate model;

use engine::execute_move;
use engine::ChessMoveError;
use model::board::row_index_to_letter;
use model::board::Board;
use model::color::Color;

pub fn execute_main_loop() {
    println!("Welcome to chess in Rust!");
    let mut chess_move_str = String::new();

    let board: Board = model::get_board();

    println!("Current board state:");
    print_board_state(&board);

    io::stdin()
        .read_line(&mut chess_move_str)
        .expect("Failed to read line");

    let result: Result<Board, ChessMoveError> = execute_move(Color::WHITE, chess_move_str, board);
    match result {
        Ok(new_board) => {
            print_board_state(&new_board);
        }
        Err(err) => {
            println!("Bad input: {:?}", err.details);
        }
    }
}

fn print_board_state(board: &Board) {
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
