use std::io;

extern crate engine;
extern crate model;

use std::env;
use engine::execute_move;
use engine::validator::ChessMoveError;
use model::board::row_index_to_letter;
use model::board::Board;
use model::color::Color;

pub fn execute_main_loop() {
    println!("Welcome to chess in Rust!");
    // Uncomment for debugging.
    env::set_var("RUST_BACKTRACE", "1");

    let mut board: Board = model::get_board();
    let mut current_color = Color::WHITE;

    println!("Current board state:");
    print_board_state(&board);

    loop {
        let mut chess_move_str = String::new();
        println!("{:?} to play: ", current_color);
        io::stdin()
            .read_line(&mut chess_move_str)
            .expect("Failed to read line");

        let result: Result<Board, ChessMoveError> =
            execute_move(&current_color, chess_move_str, board);
        match result {
            Ok(new_board) => {
                board = new_board;
                current_color = match current_color {
                    Color::WHITE => Color::BLACK,
                    Color::BLACK => Color::WHITE,
                };
                print_board_state(&board);
            }
            Err(err) => {
                board = err.board;
                println!("Bad input: {:?}", err.details);
            }
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
