// Unit tests
#[cfg(test)]
mod tic_tac_toe_tests {

    use rust_tac_toe::tic_tac_toe::TicTacToe;

    #[test]
    fn should_initiate_the_game_with_empty_board() {
        let instance = TicTacToe::new();

        for i in 0..3 {
            for j in 0..3 {
                let position = instance.board.get_position(i, j);
                assert_eq!(position, None);
            }
        }
    }
}
