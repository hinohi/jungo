use jungo::ai::{Mcts, MonteCarloAI, RandomAI};
use jungo::board::Stone;
use jungo::game::Game;
use jungo::player::Player;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::time::Instant;

fn play_game(
    player1: &dyn Player,
    player2: &dyn Player,
    board_size: usize,
) -> (i32, i32, u32, f64) {
    let mut game = Game::new(board_size);
    let start_time = Instant::now();
    let mut move_count = 0;

    loop {
        let current_player: &dyn Player = match game.current_turn {
            Stone::Black => player1,
            Stone::White => player2,
        };

        match current_player.get_move(&game.board, game.current_turn) {
            Some((x, y)) => {
                // Check if the move is valid
                if !game.board.is_valid_move(x, y, game.current_turn) {
                    continue;
                }

                // Clone board to test the move
                let mut test_board = game.board.clone();
                if test_board.place_stone(x, y, game.current_turn).is_ok() {
                    let new_hash = test_board.get_hash();

                    // Check Ko rule
                    let history_len = game.board_history.len();
                    if history_len >= 2 && game.board_history[history_len - 2] == new_hash {
                        continue; // Ko rule violation
                    }

                    // Move is valid, apply it
                    let board_before_move = game.board.clone();
                    if game.board.place_stone(x, y, game.current_turn).is_ok() {
                        game.consecutive_passes = 0;
                        game.previous_board = Some(board_before_move);
                        game.board_history.push(game.board.get_hash());
                        move_count += 1;
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
    }

    let duration = start_time.elapsed().as_secs_f64();

    // Calculate final score
    let (black_stones, white_stones) = game.board.count_stones();
    let (black_captured, white_captured) = game.board.get_captured();
    let black_score = (black_stones + black_captured) as i32;
    let white_score = (white_stones + white_captured) as i32;

    (black_score, white_score, move_count, duration)
}

fn write_result(filename: &str, content: &str) {
    create_dir_all("mcts_results").unwrap();
    let path = format!("mcts_results/{}", filename);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    writeln!(file, "{}", content).unwrap();
}

fn main() {
    println!("=== MCTS Strength Test ===");
    println!("Testing MCTS with different time limits...\n");

    // Test configurations
    let mcts_times = vec![1, 2, 3, 5, 10]; // seconds
    let board_size = 9;
    let games_per_matchup = 10;

    // Create result summary file
    write_result(
        "summary.txt",
        &format!("MCTS Strength Test - Started at {}", chrono::Local::now()),
    );
    write_result(
        "summary.txt",
        &format!("Board size: {}x{}", board_size, board_size),
    );
    write_result(
        "summary.txt",
        &format!("Games per matchup: {}\n", games_per_matchup),
    );

    // Test 1: MCTS vs Random AI
    println!("=== Test 1: MCTS vs Random AI ===");
    write_result(
        "vs_random.csv",
        "mcts_time,wins,losses,draws,win_rate,avg_score_diff",
    );

    for &mcts_time in &mcts_times {
        println!("\nTesting MCTS {}s vs Random AI...", mcts_time);
        let mcts = Mcts::new(mcts_time);
        let random = RandomAI::new();

        let mut wins = 0;
        let mut losses = 0;
        let mut draws = 0;
        let mut total_score_diff = 0;

        for game_num in 0..games_per_matchup {
            print!("Game {}/{}: ", game_num + 1, games_per_matchup);

            // Alternate who plays first
            let (black_score, white_score, _moves, _duration) = if game_num % 2 == 0 {
                play_game(&mcts, &random, board_size)
            } else {
                let (w, b, m, d) = play_game(&random, &mcts, board_size);
                (b, w, m, d) // Swap scores since we swapped players
            };

            let score_diff = black_score - white_score;
            total_score_diff += score_diff;

            if score_diff > 0 {
                wins += 1;
                println!("Win (+{})", score_diff);
            } else if score_diff < 0 {
                losses += 1;
                println!("Loss ({})", score_diff);
            } else {
                draws += 1;
                println!("Draw");
            }
        }

        let win_rate = wins as f64 / games_per_matchup as f64 * 100.0;
        let avg_score_diff = total_score_diff as f64 / games_per_matchup as f64;

        write_result(
            "vs_random.csv",
            &format!(
                "{},{},{},{},{:.1},{:.1}",
                mcts_time, wins, losses, draws, win_rate, avg_score_diff
            ),
        );

        println!(
            "Results: {} wins, {} losses, {} draws (Win rate: {:.1}%)",
            wins, losses, draws, win_rate
        );
    }

    // Test 2: MCTS vs Monte Carlo AI (1s)
    println!("\n\n=== Test 2: MCTS vs Monte Carlo AI (1s) ===");
    write_result(
        "vs_mc1s.csv",
        "mcts_time,wins,losses,draws,win_rate,avg_score_diff",
    );

    let mc1s = MonteCarloAI::new(1);
    for &mcts_time in &mcts_times {
        println!("\nTesting MCTS {}s vs Monte Carlo AI 1s...", mcts_time);
        let mcts = Mcts::new(mcts_time);

        let mut wins = 0;
        let mut losses = 0;
        let mut draws = 0;
        let mut total_score_diff = 0;

        for game_num in 0..games_per_matchup {
            print!("Game {}/{}: ", game_num + 1, games_per_matchup);

            let (black_score, white_score, _moves, _duration) = if game_num % 2 == 0 {
                play_game(&mcts, &mc1s, board_size)
            } else {
                let (w, b, m, d) = play_game(&mc1s, &mcts, board_size);
                (b, w, m, d)
            };

            let score_diff = black_score - white_score;
            total_score_diff += score_diff;

            if score_diff > 0 {
                wins += 1;
                println!("Win (+{})", score_diff);
            } else if score_diff < 0 {
                losses += 1;
                println!("Loss ({})", score_diff);
            } else {
                draws += 1;
                println!("Draw");
            }
        }

        let win_rate = wins as f64 / games_per_matchup as f64 * 100.0;
        let avg_score_diff = total_score_diff as f64 / games_per_matchup as f64;

        write_result(
            "vs_mc1s.csv",
            &format!(
                "{},{},{},{},{:.1},{:.1}",
                mcts_time, wins, losses, draws, win_rate, avg_score_diff
            ),
        );

        println!(
            "Results: {} wins, {} losses, {} draws (Win rate: {:.1}%)",
            wins, losses, draws, win_rate
        );
    }

    // Test 3: MCTS vs MCTS (different times)
    println!("\n\n=== Test 3: MCTS vs MCTS (1s as baseline) ===");
    write_result(
        "vs_mcts1s.csv",
        "mcts_time,wins,losses,draws,win_rate,avg_score_diff",
    );

    let mcts_baseline = Mcts::new(1);
    for &mcts_time in &mcts_times {
        if mcts_time == 1 {
            continue; // Skip self-play
        }

        println!("\nTesting MCTS {}s vs MCTS 1s...", mcts_time);
        let mcts = Mcts::new(mcts_time);

        let mut wins = 0;
        let mut losses = 0;
        let mut draws = 0;
        let mut total_score_diff = 0;

        for game_num in 0..games_per_matchup {
            print!("Game {}/{}: ", game_num + 1, games_per_matchup);

            let (black_score, white_score, _moves, _duration) = if game_num % 2 == 0 {
                play_game(&mcts, &mcts_baseline, board_size)
            } else {
                let (w, b, m, d) = play_game(&mcts_baseline, &mcts, board_size);
                (b, w, m, d)
            };

            let score_diff = black_score - white_score;
            total_score_diff += score_diff;

            if score_diff > 0 {
                wins += 1;
                println!("Win (+{})", score_diff);
            } else if score_diff < 0 {
                losses += 1;
                println!("Loss ({})", score_diff);
            } else {
                draws += 1;
                println!("Draw");
            }
        }

        let win_rate = wins as f64 / games_per_matchup as f64 * 100.0;
        let avg_score_diff = total_score_diff as f64 / games_per_matchup as f64;

        write_result(
            "vs_mcts1s.csv",
            &format!(
                "{},{},{},{},{:.1},{:.1}",
                mcts_time, wins, losses, draws, win_rate, avg_score_diff
            ),
        );

        println!(
            "Results: {} wins, {} losses, {} draws (Win rate: {:.1}%)",
            wins, losses, draws, win_rate
        );
    }

    // Generate final summary
    println!("\n\n=== Generating Summary ===");
    write_result("summary.txt", "\n=== FINAL SUMMARY ===");

    // Read and summarize results
    println!("\nTest completed! Results saved in mcts_results/");
    println!("Files created:");
    println!("  - summary.txt");
    println!("  - vs_random.csv");
    println!("  - vs_mc1s.csv");
    println!("  - vs_mcts1s.csv");
}
