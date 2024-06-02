mod board;
mod tic_tac_toe;

use crate::tic_tac_toe::TicTacToe;

fn main() {
    println!("Hello, Tic Tac Toe");

    match TicTacToe::new().start_game() {
        Err(v) => {
            println!("Something went wrong {}", v.to_string())
        }
        Ok(()) => {
            println!("Game Over!")
        }
    }
}
