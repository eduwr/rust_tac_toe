// Unit tests
#[cfg(test)]
mod board_tests {
    use rust_tac_toe::board::{Board, Player, Position};

    #[test]
    fn should_make_move() {
        let mut board = Board::new();
        assert!(board.make_move(Position::new(0, 0), Player::X).is_ok());
        assert!(board.make_move(Position::new(1, 1), Player::O).is_ok());
        assert!(board.make_move(Position::new(0, 0), Player::X).is_err());

        assert_eq!(board.get_position(1, 1), Some(Player::O));
        assert_eq!(board.get_position(0, 0), Some(Player::X));
    }

    fn build_board((player1_moves, player2_moves): (Vec<Position>, Vec<Position>)) -> Board {
        let mut board = Board::new();

        for (player_move, opponent_move) in player1_moves.iter().zip(player2_moves.iter()) {
            let _ = board.make_move(*player_move, Player::O);
            let _ = board.make_move(*opponent_move, Player::X);
        }

        board
    }

    #[test]
    fn should_get_winner_when_vertical_condition_met() {
        let vertical_winner = (0..3).map(|i| Position::new(i, 1)).collect();
        let vertical_loser = vec![
            Position::new(1, 0),
            Position::new(1, 2),
            Position::new(2, 2),
        ];

        let board = build_board((vertical_winner, vertical_loser));
        assert_eq!(board.has_winner(), Some(Player::O));
    }

    #[test]
    fn should_get_winner_when_horizontal_condition_met() {
        let horizontal_winner = (0..3).map(|i| Position::new(0, i)).collect();
        let horizontal_loser: Vec<Position> = vec![
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(2, 1),
        ];

        let board = build_board((horizontal_winner, horizontal_loser));
        assert_eq!(board.has_winner(), Some(Player::O));
    }

    #[test]
    fn should_get_winner_when_diagonal_condition_met() {
        let diagonal_winner = (0..3).map(|i| Position::new(i, i)).collect();
        let diagonal_loser = vec![
            Position::new(0, 2),
            Position::new(0, 1),
            Position::new(1, 0),
        ];

        let board = build_board((diagonal_loser, diagonal_winner));
        assert_eq!(board.has_winner(), Some(Player::X));
    }

    #[test]
    fn should_get_winner_when_counter_diagonal_condition_met() {
        let diagonal_winner = (0..3).map(|i| Position::new(i, 2 - i)).collect();
        let diagonal_loser = vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(1, 0),
        ];

        let board = build_board((diagonal_loser, diagonal_winner));
        assert_eq!(board.has_winner(), Some(Player::X));
    }

    #[test]
    fn should_return_none_when_there_is_no_winner() {
        let player_moves = vec![
            Position::new(0, 0),
            Position::new(1, 1),
            Position::new(0, 2),
        ];
        let opponent_moves = vec![
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(2, 2),
        ];

        let board = build_board((player_moves, opponent_moves));

        assert_eq!(board.has_winner(), None);
    }
}
