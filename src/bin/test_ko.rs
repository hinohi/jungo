use jungo::board::Stone;
use jungo::game::Game;

fn main() {
    println!("Testing Ko detection with Zobrist hashing");

    // Create a classic Ko situation
    let mut game = Game::new(5);

    // Setup the classic Ko pattern
    // Black stones
    game.board.place_stone(1, 1, Stone::Black).unwrap();
    game.board.place_stone(2, 0, Stone::Black).unwrap();
    game.board.place_stone(3, 1, Stone::Black).unwrap();
    game.board.place_stone(2, 2, Stone::Black).unwrap();

    // White stones
    game.board.place_stone(0, 1, Stone::White).unwrap();
    game.board.place_stone(1, 0, Stone::White).unwrap();
    game.board.place_stone(1, 2, Stone::White).unwrap();

    // Save initial board state
    let initial_hash = game.board.get_hash();
    game.board_history.clear();
    game.board_history.push(initial_hash);

    println!("Initial board (before Ko situation):");
    println!("{}", game.board);
    println!("Hash: {}", initial_hash);

    // White captures Black at (1,1)
    println!("\nWhite captures Black at (2,1):");
    game.board.place_stone(2, 1, Stone::White).unwrap();
    let after_white_capture = game.board.get_hash();
    game.board_history.push(after_white_capture);
    println!("{}", game.board);
    println!("Hash: {}", after_white_capture);

    // Black wants to immediately recapture at (2,1) - this creates Ko
    println!("\nBlack tries to recapture at (1,1):");

    // Simulate the move
    let mut test_board = game.board.clone();
    if test_board.place_stone(1, 1, Stone::Black).is_ok() {
        let new_hash = test_board.get_hash();
        println!("Move would result in hash: {}", new_hash);

        // Check Ko rule - see if this recreates board from 2 moves ago
        let history_len = game.board_history.len();
        if history_len >= 2 {
            let hash_two_moves_ago = game.board_history[history_len - 2];
            println!("Hash from 2 moves ago: {}", hash_two_moves_ago);

            if new_hash == hash_two_moves_ago {
                println!("\n✗ Ko rule violation! Move blocked.");
                println!("This move would recreate the board position from 2 moves ago.");
            } else {
                println!("\n✓ Move is allowed (no Ko violation)");
                println!("Board after move:");
                println!("{}", test_board);
            }
        }
    }

    // Show that after another move, the Ko is resolved
    println!("\n--- After Black plays elsewhere, Ko is resolved ---");
    game.board.place_stone(4, 4, Stone::Black).unwrap();
    game.board_history.push(game.board.get_hash());

    println!("\nNow Black can recapture at (1,1):");
    let mut test_board2 = game.board.clone();
    if test_board2.place_stone(1, 1, Stone::Black).is_ok() {
        let new_hash = test_board2.get_hash();
        let history_len = game.board_history.len();

        if history_len >= 2 && new_hash == game.board_history[history_len - 2] {
            println!("✗ Ko rule violation!");
        } else {
            println!("✓ Move is now allowed! Ko is resolved.");
        }
    }
}
