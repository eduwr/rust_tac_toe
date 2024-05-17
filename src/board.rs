#[derive(Copy, Clone, Debug)]
pub enum Player {
    O,
    X,
}

pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Board {
    game: [[Option<Player>; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Board {
            game: [[None; 3]; 3],
        }
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
}
