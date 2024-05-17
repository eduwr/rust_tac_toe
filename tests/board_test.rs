// Unit tests
#[cfg(test)]
mod board_tests {
    use rust_tac_toe::board::{Board, Player, Position};

    use super::*;

    #[test]
    fn should_make_move() {
        println!("Test starting");
        let mut board = Board::new();
        assert!(board
            .make_move(Position { col: 0, row: 0 }, Player::X)
            .is_ok());
        assert!(board
            .make_move(Position { col: 1, row: 1 }, Player::O)
            .is_ok());
        assert!(board
            .make_move(Position { col: 0, row: 0 }, Player::X)
            .is_err());
    }
}
