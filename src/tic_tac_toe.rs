use crate::board::{Board, Position};
use std::{io, num::ParseIntError};

fn read_coordinates() -> io::Result<()> {
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
        return Ok(());
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

    pub fn start_game() {
        let _ = read_coordinates();

        // Logic here
    }
}
