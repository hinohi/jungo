use jungo::ai::{Mcts, MonteCarloAI};
use jungo::board::Stone;
use jungo::game::Game;
use jungo::player::Player;
use std::time::Instant;

fn play_game_with_mercy(
    player1: &dyn Player,
    player2: &dyn Player,
    board_size: usize,
) -> (i32, i32, bool) {
    let mut game = Game::new(board_size);
    let mut move_count = 0;
    let mercy_threshold = (board_size * board_size) as i32 / 2; // Early termination if score difference > half board size
    let mut was_mercy_ruled = false;

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
                        move_count += 1;
                        game.consecutive_passes = 0;
                        game.previous_board = Some(board_before_move);
                        game.board_history.push(game.board.get_hash());
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

        // Check for mercy rule every 10 moves after move 20
        if move_count > 20 && move_count % 10 == 0 {
            let (black_stones, white_stones) = game.board.count_stones();
            let (black_captured, white_captured) = game.board.get_captured();
            let black_score = (black_stones + black_captured) as i32;
            let white_score = (white_stones + white_captured) as i32;
            let score_diff = (black_score - white_score).abs();

            if score_diff > mercy_threshold {
                println!(
                    "  Mercy rule activated at move {} (score diff: {})",
                    move_count, score_diff
                );
                was_mercy_ruled = true;
                break;
            }
        }

        game.current_turn = game.current_turn.opposite();
    }

    let (black_stones, white_stones) = game.board.count_stones();
    let (black_captured, white_captured) = game.board.get_captured();
    let black_score = (black_stones + black_captured) as i32;
    let white_score = (white_stones + white_captured) as i32;

    (black_score, white_score, was_mercy_ruled)
}

fn test_mcts_performance(mcts_time_ms: u64) {
    println!("\n=== Testing MCTS {} ms ===", mcts_time_ms);

    let mc_fast = MonteCarloAI::new_with_millis(100);
    let mcts = Mcts::new_with_millis(mcts_time_ms);

    let mut mcts_wins = 0;
    let mut mc_wins = 0;
    let mut mercy_count = 0;

    for game_num in 0..4 {
        print!("Game {}: ", game_num + 1);
        let start = Instant::now();

        let (black_score, white_score, mercy) = if game_num % 2 == 0 {
            // MCTS as Black
            play_game_with_mercy(&mcts, &mc_fast, 5)
        } else {
            // MCTS as White
            let (w, b, m) = play_game_with_mercy(&mc_fast, &mcts, 5);
            (b, w, m)
        };

        if mercy {
            mercy_count += 1;
        }

        let mcts_score = if game_num % 2 == 0 {
            black_score
        } else {
            white_score
        };
        let mc_score = if game_num % 2 == 0 {
            white_score
        } else {
            black_score
        };

        if mcts_score > mc_score {
            mcts_wins += 1;
            println!(
                "MCTS wins {}-{} ({:.1}s)",
                mcts_score,
                mc_score,
                start.elapsed().as_secs_f64()
            );
        } else {
            mc_wins += 1;
            println!(
                "MC wins {}-{} ({:.1}s)",
                mc_score,
                mcts_score,
                start.elapsed().as_secs_f64()
            );
        }
    }

    println!("\nResults: MCTS {}-{} MC(0.1s)", mcts_wins, mc_wins);
    if mercy_count > 0 {
        println!("Games ended by mercy rule: {}/4", mercy_count);
    }
}

fn main() {
    println!("=== MCTS Performance Testing ===");
    println!("Testing against MC(0.1s) as baseline");
    println!("4 games per test (2 as Black, 2 as White)");
    println!(
        "Mercy rule: Game ends if score difference > {} points\n",
        5 * 5 / 2
    );

    // Test current MCTS implementation
    test_mcts_performance(100);
    test_mcts_performance(500);
    test_mcts_performance(1000);

    println!("\n=== Summary ===");
    println!("Current MCTS implementation needs improvement.");
    println!("Next steps: Debug UCT calculation and tree building.");
}
