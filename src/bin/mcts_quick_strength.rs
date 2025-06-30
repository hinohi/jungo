use jungo::ai::{Mcts, MonteCarloAI, RandomAI};
use jungo::board::Stone;
use jungo::game::Game;
use jungo::player::Player;
use std::fs::{create_dir_all, File};
use std::io::Write;

fn play_game(player1: &dyn Player, player2: &dyn Player, board_size: usize) -> (i32, i32) {
    let mut game = Game::new(board_size);

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

    let (black_stones, white_stones) = game.board.count_stones();
    let (black_captured, white_captured) = game.board.get_captured();
    let black_score = (black_stones + black_captured) as i32;
    let white_score = (white_stones + white_captured) as i32;

    (black_score, white_score)
}

fn main() {
    println!("=== MCTS Quick Strength Test ===");
    println!("Testing MCTS with different time limits (fewer games)...\n");

    let mcts_times = vec![1, 3, 5];
    let board_size = 9;
    let games_per_matchup = 4; // Reduced for quick test

    create_dir_all("mcts_results").unwrap();
    let mut summary = File::create("mcts_results/quick_summary.txt").unwrap();

    writeln!(summary, "MCTS Quick Strength Test").unwrap();
    writeln!(summary, "Board size: {}x{}", board_size, board_size).unwrap();
    writeln!(summary, "Games per matchup: {}\n", games_per_matchup).unwrap();

    // Test 1: MCTS vs Random AI
    println!("=== MCTS vs Random AI ===");
    writeln!(summary, "\n=== MCTS vs Random AI ===").unwrap();
    writeln!(summary, "Time | Wins | Losses | Win Rate").unwrap();
    writeln!(summary, "-----|------|--------|----------").unwrap();

    for &mcts_time in &mcts_times {
        print!("MCTS {}s: ", mcts_time);
        let mcts = Mcts::new(mcts_time);
        let random = RandomAI::new();

        let mut wins = 0;
        for i in 0..games_per_matchup {
            let (black_score, white_score) = if i % 2 == 0 {
                play_game(&mcts, &random, board_size)
            } else {
                let (w, b) = play_game(&random, &mcts, board_size);
                (b, w)
            };

            if black_score > white_score {
                wins += 1;
                print!("W");
            } else {
                print!("L");
            }
            std::io::stdout().flush().unwrap();
        }

        let win_rate = wins as f64 / games_per_matchup as f64 * 100.0;
        println!(" => {}/{} ({:.0}%)", wins, games_per_matchup, win_rate);
        writeln!(
            summary,
            "{:4}s | {:4} | {:6} | {:8.0}%",
            mcts_time,
            wins,
            games_per_matchup - wins,
            win_rate
        )
        .unwrap();
    }

    // Test 2: MCTS vs Monte Carlo 1s
    println!("\n=== MCTS vs Monte Carlo AI (1s) ===");
    writeln!(summary, "\n=== MCTS vs Monte Carlo AI (1s) ===").unwrap();
    writeln!(summary, "Time | Wins | Losses | Win Rate").unwrap();
    writeln!(summary, "-----|------|--------|----------").unwrap();

    let mc1s = MonteCarloAI::new(1);
    for &mcts_time in &mcts_times {
        print!("MCTS {}s: ", mcts_time);
        let mcts = Mcts::new(mcts_time);

        let mut wins = 0;
        for i in 0..games_per_matchup {
            let (black_score, white_score) = if i % 2 == 0 {
                play_game(&mcts, &mc1s, board_size)
            } else {
                let (w, b) = play_game(&mc1s, &mcts, board_size);
                (b, w)
            };

            if black_score > white_score {
                wins += 1;
                print!("W");
            } else {
                print!("L");
            }
            std::io::stdout().flush().unwrap();
        }

        let win_rate = wins as f64 / games_per_matchup as f64 * 100.0;
        println!(" => {}/{} ({:.0}%)", wins, games_per_matchup, win_rate);
        writeln!(
            summary,
            "{:4}s | {:4} | {:6} | {:8.0}%",
            mcts_time,
            wins,
            games_per_matchup - wins,
            win_rate
        )
        .unwrap();
    }

    // Test 3: MCTS times comparison
    println!("\n=== MCTS 1s vs MCTS (other times) ===");
    writeln!(summary, "\n=== MCTS 1s vs MCTS (other times) ===").unwrap();
    writeln!(summary, "Time | Wins vs 1s | Losses | Win Rate").unwrap();
    writeln!(summary, "-----|------------|--------|----------").unwrap();

    let mcts1s = Mcts::new(1);
    for &mcts_time in &[3, 5] {
        print!("MCTS {}s: ", mcts_time);
        let mcts = Mcts::new(mcts_time);

        let mut wins = 0;
        for i in 0..games_per_matchup {
            let (black_score, white_score) = if i % 2 == 0 {
                play_game(&mcts, &mcts1s, board_size)
            } else {
                let (w, b) = play_game(&mcts1s, &mcts, board_size);
                (b, w)
            };

            if black_score > white_score {
                wins += 1;
                print!("W");
            } else {
                print!("L");
            }
            std::io::stdout().flush().unwrap();
        }

        let win_rate = wins as f64 / games_per_matchup as f64 * 100.0;
        println!(" => {}/{} ({:.0}%)", wins, games_per_matchup, win_rate);
        writeln!(
            summary,
            "{:4}s | {:10} | {:6} | {:8.0}%",
            mcts_time,
            wins,
            games_per_matchup - wins,
            win_rate
        )
        .unwrap();
    }

    println!("\n=== Summary saved to mcts_results/quick_summary.txt ===");
}
