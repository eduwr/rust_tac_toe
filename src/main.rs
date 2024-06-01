mod board;

use rust_tac_toe::tic_tac_toe::TicTacToe;

fn main() {
    println!("Hello, Tic Tac Toe");

    let mut game = TicTacToe::new();
    TicTacToe::start_game(&mut game);
}
