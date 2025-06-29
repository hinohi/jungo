use jungo::ai::Mcts;
use jungo::board::{Board, Stone};
use jungo::player::Player;

fn main() {
    println!("=== Simple MCTS Test ===\n");

    // Create a simple board position
    let mut board = Board::new(5);

    // Place some stones to create a simple position
    board.place_stone(1, 1, Stone::Black).unwrap();
    board.place_stone(2, 1, Stone::White).unwrap();
    board.place_stone(1, 2, Stone::Black).unwrap();
    board.place_stone(2, 2, Stone::White).unwrap();

    println!("Current board:");
    println!("{}", board);

    // Create MCTS with short time limit for testing
    let mcts = Mcts::new(1);

    // Get MCTS move for Black
    println!("\nMCTS thinking for Black...");
    if let Some((x, y)) = mcts.get_move(&board, Stone::Black) {
        println!("MCTS suggests move: {}{}", (b'A' + x as u8) as char, 5 - y);
    } else {
        println!("MCTS suggests pass");
    }

    // Test a few more times to see consistency
    println!("\nTesting consistency (5 runs):");
    for i in 1..=5 {
        if let Some((x, y)) = mcts.get_move(&board, Stone::Black) {
            println!("Run {}: {}{}", i, (b'A' + x as u8) as char, 5 - y);
        } else {
            println!("Run {}: pass", i);
        }
    }
}
