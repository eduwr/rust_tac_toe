mod board;

use board::{Board, Position, Player};


fn main() {
    println!("Hello, Tic Tac Toe");
    let mut b = Board::new();

    b.show();
    println!();
    b.make_move(Position { row: 1, col: 1 }, Player::O).unwrap();
    b.show();
}

// [[i, j, k], [l, m , n] , [o, p q]]
