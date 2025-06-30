use jungo::ai::{Mcts, MonteCarloAI, RandomAI};
use jungo::board::Stone;
use jungo::game::Game;
use jungo::player::Player;

fn play_match(player1: &dyn Player, player2: &dyn Player, board_size: usize) -> String {
    let mut game = Game::new(board_size);
    let mut moves = 0;

    loop {
        let current_player: &dyn Player = match game.current_turn {
            Stone::Black => player1,
            Stone::White => player2,
        };

        match current_player.get_move(&game.board, game.current_turn) {
            Some((x, y)) => {
                if !game.board.is_valid_move(x, y, game.current_turn) {
                    continue;
                }

                let mut test_board = game.board.clone();
                if test_board.place_stone(x, y, game.current_turn).is_ok() {
                    let new_hash = test_board.get_hash();
                    let history_len = game.board_history.len();
                    if history_len >= 2 && game.board_history[history_len - 2] == new_hash {
                        continue;
                    }

                    let board_before_move = game.board.clone();
                    if game.board.place_stone(x, y, game.current_turn).is_ok() {
                        game.consecutive_passes = 0;
                        game.previous_board = Some(board_before_move);
                        game.board_history.push(game.board.get_hash());
                        moves += 1;
                    }
                }
            }
            None => {
                game.consecutive_passes += 1;
                if game.consecutive_passes >= 2 {
                    break;
                }
            }
        }

        game.current_turn = game.current_turn.opposite();

        if moves > 100 {
            break; // Prevent infinite games
        }
    }

    let (black_stones, white_stones) = game.board.count_stones();
    let (black_captured, white_captured) = game.board.get_captured();
    let black_score = black_stones + black_captured;
    let white_score = white_stones + white_captured;

    format!(
        "B:{} W:{} ({})",
        black_score,
        white_score,
        if black_score > white_score {
            "Black wins"
        } else if white_score > black_score {
            "White wins"
        } else {
            "Draw"
        }
    )
}

fn main() {
    println!("=== Simple MCTS Match Test ===\n");

    // Test different matchups on 5x5 board
    let board_size = 5;

    println!("Testing on {}x{} board:", board_size, board_size);
    println!("B = Black (first player), W = White (second player)\n");

    // Create AIs
    let random = RandomAI::new();
    let mc1s = MonteCarloAI::new(1);
    let mcts1s = Mcts::new(1);
    let mcts3s = Mcts::new(3);

    // Test 1: Random vs Random (baseline)
    println!("1. Random vs Random:");
    println!("   Game 1: {}", play_match(&random, &random, board_size));
    println!("   Game 2: {}", play_match(&random, &random, board_size));

    // Test 2: MCTS 1s vs Random
    println!("\n2. MCTS 1s vs Random:");
    println!(
        "   Game 1 (MCTS=B): {}",
        play_match(&mcts1s, &random, board_size)
    );
    println!(
        "   Game 2 (Random=B): {}",
        play_match(&random, &mcts1s, board_size)
    );

    // Test 3: MCTS 3s vs Random
    println!("\n3. MCTS 3s vs Random:");
    println!(
        "   Game 1 (MCTS=B): {}",
        play_match(&mcts3s, &random, board_size)
    );
    println!(
        "   Game 2 (Random=B): {}",
        play_match(&random, &mcts3s, board_size)
    );

    // Test 4: MCTS vs Monte Carlo
    println!("\n4. MCTS 1s vs Monte Carlo 1s:");
    println!(
        "   Game 1 (MCTS=B): {}",
        play_match(&mcts1s, &mc1s, board_size)
    );
    println!(
        "   Game 2 (MC=B): {}",
        play_match(&mc1s, &mcts1s, board_size)
    );

    // Test 5: MCTS 3s vs MCTS 1s
    println!("\n5. MCTS 3s vs MCTS 1s:");
    println!(
        "   Game 1 (3s=B): {}",
        play_match(&mcts3s, &mcts1s, board_size)
    );
    println!(
        "   Game 2 (1s=B): {}",
        play_match(&mcts1s, &mcts3s, board_size)
    );

    println!("\n=== Test Complete ===");
}
