const SIZE: usize = 3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    O,
    X,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

#[derive(Debug)]
pub struct Board {
    game: [[Option<Player>; SIZE]; SIZE],
}

impl Board {
    pub fn new() -> Self {
        Board {
            game: [[None; SIZE]; SIZE],
        }
    }

    pub fn get_position(&self, row: usize, col: usize) -> Option<Player> {
        self.game[row][col]
    }

    pub fn make_move(&mut self, position: Position, player: Player) -> Result<(), &str> {
        let row = position.row;
        let col = position.col;

        if row > 3 || col > 3 {
            return Err("out of range");
        }

        match self.game[row][col] {
            Some(_) => {
                return Err("already taken");
            }
            None => {
                self.game[row][col] = Some(player);
            }
        }

        Ok(())
    }

    pub fn show(&self) {
        let rows = self
            .game
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&col| match col {
                        None => "   ",
                        Some(Player::O) => " O ",
                        Some(Player::X) => " X ",
                    })
                    .collect::<Vec<_>>()
                    .join("|")
            })
            .collect::<Vec<_>>()
            .join("\n-----------\n");

        println!("{}", rows)
    }

    pub fn has_winner(&self) -> Option<Player> {
        for i in 0..SIZE {
            // check winner in the rows
            match (
                self.get_position(i, 0),
                self.get_position(i, 1),
                self.get_position(i, 2),
            ) {
                (Some(p0), Some(p1), Some(p2)) if p0 == p1 && p1 == p2 => return Some(p0),
                _ => {}
            }

            // check for a winner along the columns
            match (
                self.get_position(0, i),
                self.get_position(1, i),
                self.get_position(2, i),
            ) {
                (Some(p0), Some(p1), Some(p2)) if p0 == p1 && p1 == p2 => return Some(p0),
                _ => {}
            }
        }

        // Check along the horizontal axis
        match (
            self.get_position(0, 0),
            self.get_position(1, 1),
            self.get_position(2, 2),
        ) {
            (Some(p0), Some(p1), Some(p2)) if p0 == p1 && p1 == p2 => return Some(p0),
            _ => {}
        }

        match (
            self.get_position(0, 2),
            self.get_position(1, 1),
            self.get_position(2, 0),
        ) {
            (Some(p0), Some(p1), Some(p2)) if p0 == p1 && p1 == p2 => return Some(p0),
            _ => {}
        }

        None
    }
}

// :row.every(player)
// O0 O1 O2
// 10 11 12
// 20 21 22

//
//

// 00 11 22
// 02 11 20
// O
//   0
//     0

// 00 01 02
// 10 11 12
// 20 21 22

// 0
// 0
// 0
