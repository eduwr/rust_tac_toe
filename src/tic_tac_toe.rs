use crate::board::{self, Board, Player, Position};
use std::{borrow::Borrow, fmt::Error, io, num::ParseIntError};

fn read_coordinates() -> io::Result<Position> {
    loop {
        println!("input the position separated by comma:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut coord: Vec<u8> = Vec::new();

        for term in input.trim().split(",") {
            if coord.len() > 2 {
                continue;
            }

            if let Ok(val) = term.parse::<u8>() {
                if val < 3 {
                    coord.push(val)
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }

        let pos = Position::new(coord[0].into(), coord[1].into());

        println!("You typed: {:?}", pos);
        return Ok(pos);
    }
}

pub struct TicTacToe {
    pub board: Board,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: Board::new(),
        }
    }

    pub fn start_game(&mut self) -> Result<(), &str> {
        let mut player = Player::O;
        loop {
            self.board.show();
            let winner = self.board.has_winner();
            if winner.is_some() {
                println!("Player, {:?} wins", winner.unwrap());
                return Ok(());
            }

            let coord = read_coordinates();

            if coord.is_err() {
                continue;
            }

            let mov = self.board.make_move(coord.unwrap(), player);

            if mov.is_err() {
                println!("{:?}", mov.err());
                continue;
            }

            player = if player == Player::O {
                Player::X
            } else {
                Player::O
            };
        }
    }
}
