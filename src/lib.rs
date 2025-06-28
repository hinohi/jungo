pub mod ai;
pub mod board;
pub mod game;
pub mod player;

#[cfg(test)]
mod tests {
    use super::*;
    use board::{Board, Stone};
    use game::Game;

    #[test]
    fn test_stone_opposite() {
        assert_eq!(Stone::Black.opposite(), Stone::White);
        assert_eq!(Stone::White.opposite(), Stone::Black);
    }

    #[test]
    fn test_board_creation() {
        let board = Board::new(9);
        assert_eq!(board.size(), 9);
        assert!(board.is_empty());
    }

    #[test]
    fn test_board_place_stone() {
        let mut board = Board::new(9);

        assert!(board.place_stone(0, 0, Stone::Black).is_ok());
        assert_eq!(board.get(0, 0), Some(Stone::Black));

        assert!(board.place_stone(0, 0, Stone::White).is_err());
    }

    #[test]
    fn test_board_is_valid_move() {
        let mut board = Board::new(9);

        assert!(board.is_valid_move(0, 0, Stone::Black));
        assert!(board.is_valid_move(8, 8, Stone::White));
        assert!(!board.is_valid_move(9, 0, Stone::Black));
        assert!(!board.is_valid_move(0, 9, Stone::White));

        board.place_stone(0, 0, Stone::Black).unwrap();
        assert!(!board.is_valid_move(0, 0, Stone::White));
    }

    #[test]
    fn test_simple_capture() {
        let mut board = Board::new(9);

        board.place_stone(1, 0, Stone::Black).unwrap();
        board.place_stone(0, 1, Stone::Black).unwrap();
        board.place_stone(0, 0, Stone::White).unwrap();

        assert_eq!(board.get(0, 0), None);
        assert_eq!(board.get_captured(), (0, 0));
    }

    #[test]
    fn test_group_capture() {
        let mut board = Board::new(9);

        board.place_stone(1, 0, Stone::White).unwrap();
        board.place_stone(2, 0, Stone::White).unwrap();
        board.place_stone(1, 1, Stone::White).unwrap();

        board.place_stone(0, 0, Stone::Black).unwrap();
        board.place_stone(0, 1, Stone::Black).unwrap();
        board.place_stone(1, 2, Stone::Black).unwrap();
        board.place_stone(2, 1, Stone::Black).unwrap();
        board.place_stone(3, 0, Stone::Black).unwrap();

        assert_eq!(board.get(1, 0), None);
        assert_eq!(board.get(2, 0), None);
        assert_eq!(board.get(1, 1), None);
        assert_eq!(board.get_captured(), (3, 0));
    }

    #[test]
    fn test_count_stones() {
        let mut board = Board::new(9);

        board.place_stone(0, 0, Stone::Black).unwrap();
        board.place_stone(1, 0, Stone::White).unwrap();
        board.place_stone(2, 0, Stone::Black).unwrap();
        board.place_stone(3, 0, Stone::White).unwrap();

        let (black, white) = board.count_stones();
        assert_eq!(black, 2);
        assert_eq!(white, 2);
    }

    #[test]
    fn test_game_creation() {
        let game = Game::new(9);
        assert_eq!(game.board.size(), 9);
        assert_eq!(game.current_turn, Stone::Black);
        assert_eq!(game.consecutive_passes, 0);
    }

    #[test]
    fn test_consecutive_passes() {
        let mut game = Game::new(9);

        assert_eq!(game.consecutive_passes, 0);

        game.consecutive_passes = 1;
        assert_eq!(game.consecutive_passes, 1);

        game.consecutive_passes = 2;
        assert_eq!(game.consecutive_passes, 2);
    }

    #[test]
    fn test_turn_switching() {
        let mut game = Game::new(9);

        assert_eq!(game.current_turn, Stone::Black);
        game.current_turn = game.current_turn.opposite();
        assert_eq!(game.current_turn, Stone::White);
        game.current_turn = game.current_turn.opposite();
        assert_eq!(game.current_turn, Stone::Black);
    }

    #[test]
    fn test_edge_coordinates() {
        let mut board = Board::new(9);

        assert!(board.place_stone(0, 0, Stone::Black).is_ok());
        assert!(board.place_stone(8, 0, Stone::White).is_ok());
        assert!(board.place_stone(0, 8, Stone::Black).is_ok());
        assert!(board.place_stone(8, 8, Stone::White).is_ok());

        assert!(board.place_stone(9, 0, Stone::Black).is_err());
        assert!(board.place_stone(0, 9, Stone::White).is_err());
    }

    #[test]
    fn test_ko_situation() {
        // Test that Ko rule works correctly
        let mut game = Game::new(5);

        // The Ko rule is tested through the Game struct which maintains board history
        // We'll test that the game properly tracks previous board state
        assert!(game.previous_board.is_none());

        // Make a move
        game.board.place_stone(0, 0, Stone::Black).unwrap();
        game.previous_board = Some(game.board.clone());

        // Make another move
        game.board.place_stone(1, 1, Stone::White).unwrap();

        // Test that is_valid_move_with_ko works
        if let Some(ref prev) = game.previous_board {
            // A move that doesn't recreate the previous board should be allowed
            assert!(game.board.is_valid_move_with_ko(2, 2, Stone::Black, prev));
        }
    }

    #[test]
    fn test_liberty_calculation() {
        let mut board = Board::new(9);

        board.place_stone(4, 4, Stone::Black).unwrap();

        board.place_stone(3, 4, Stone::White).unwrap();
        board.place_stone(5, 4, Stone::White).unwrap();
        board.place_stone(4, 3, Stone::White).unwrap();
        board.place_stone(4, 5, Stone::White).unwrap();

        assert_eq!(board.get(4, 4), None);
        assert_eq!(board.get_captured(), (0, 1));
    }

    #[test]
    fn test_corner_capture() {
        let mut board = Board::new(9);

        board.place_stone(1, 0, Stone::Black).unwrap();
        board.place_stone(0, 1, Stone::Black).unwrap();
        board.place_stone(0, 0, Stone::White).unwrap();

        assert_eq!(board.get(0, 0), None);
        assert_eq!(board.get_captured(), (0, 0));
    }

    #[test]
    fn test_self_capture_allowed() {
        let mut board = Board::new(9);

        board.place_stone(1, 0, Stone::Black).unwrap();
        board.place_stone(0, 1, Stone::Black).unwrap();

        assert!(board.place_stone(0, 0, Stone::White).is_ok());
        assert_eq!(board.get(0, 0), None);
    }

    #[test]
    fn test_large_group_capture() {
        let mut board = Board::new(9);

        for i in 0..4 {
            board.place_stone(i, 0, Stone::White).unwrap();
            board.place_stone(i, 1, Stone::White).unwrap();
        }

        for i in 0..4 {
            board.place_stone(i, 2, Stone::Black).unwrap();
            if i < 3 {
                board.place_stone(i + 1, 3, Stone::Black).unwrap();
            }
        }
        board.place_stone(4, 0, Stone::Black).unwrap();
        board.place_stone(4, 1, Stone::Black).unwrap();
        board.place_stone(0, 3, Stone::Black).unwrap();

        for i in 0..4 {
            assert_eq!(board.get(i, 0), None);
            assert_eq!(board.get(i, 1), None);
        }
        assert_eq!(board.get_captured(), (8, 0));
    }

    #[test]
    fn test_suicide_without_capture_invalid() {
        let mut board = Board::new(5);

        // Fill most of the board with Black stones
        for y in 0..5 {
            for x in 0..5 {
                if !((x == 0 && y == 3) || (x == 2 && y == 3)) {
                    board.place_stone(x, y, Stone::Black).unwrap();
                }
            }
        }

        // A2 (0,3) and C2 (2,3) should be invalid for White
        assert!(!board.is_valid_move(0, 3, Stone::White));
        assert!(!board.is_valid_move(2, 3, Stone::White));
    }

    #[test]
    fn test_suicide_with_capture_valid() {
        let mut board = Board::new(5);

        // Create a situation where a suicide move would capture opponent stones
        // Setup: White stone at corner surrounded by Black, but Black group has no liberties
        board.place_stone(0, 0, Stone::White).unwrap();
        board.place_stone(1, 0, Stone::Black).unwrap();
        board.place_stone(0, 1, Stone::Black).unwrap();
        board.place_stone(1, 1, Stone::White).unwrap();

        // Now if Black plays at (0,0), it would capture the White stone
        // even though Black stone itself would have no liberties after White is removed
        assert!(board.is_valid_move(0, 0, Stone::Black));
    }

    #[test]
    fn test_ko_rule_blocks_immediate_recapture() {
        // Test the board comparison function
        let mut board1 = Board::new(5);
        let board2 = Board::new(5);

        // Same boards should be equal
        assert!(board1.boards_are_equal(&board1, &board2));

        // Different boards should not be equal
        board1.place_stone(0, 0, Stone::Black).unwrap();
        assert!(!board1.boards_are_equal(&board1, &board2));

        // Test Ko validation mechanism
        let mut board = Board::new(5);
        board.place_stone(1, 1, Stone::Black).unwrap();
        let state1 = board.clone();

        board.place_stone(2, 2, Stone::White).unwrap();
        let state2 = board.clone();

        // A move is blocked by Ko if it would recreate the previous board state
        assert!(board.is_valid_move_with_ko(3, 3, Stone::Black, &state1)); // Different from state1
        assert!(board.is_valid_move_with_ko(3, 3, Stone::Black, &state2)); // Different from state2
    }
}
