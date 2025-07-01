use jungo::ai::{Mcts, MonteCarloAI};
use jungo::board::Stone;
use jungo::game::Game;
use jungo::player::Player;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::time::Instant;

fn play_game(player1: &dyn Player, player2: &dyn Player, board_size: usize) -> (i32, i32, usize) {
    let mut game = Game::new(board_size);
    let mut move_count = 0;

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

        game.current_turn = game.current_turn.opposite();

        if move_count > 100 {
            break;
        }
    }

    let (black_stones, white_stones) = game.board.count_stones();
    let (black_captured, white_captured) = game.board.get_captured();
    let black_score = (black_stones + black_captured) as i32;
    let white_score = (white_stones + white_captured) as i32;

    (black_score, white_score, move_count)
}

struct MatchResult {
    player1_name: String,
    player2_name: String,
    player1_wins: i32,
    player2_wins: i32,
    draws: i32,
    total_games: i32,
    avg_moves: f64,
}

fn run_match(
    player1: &dyn Player,
    player2: &dyn Player,
    games_per_match: usize,
    board_size: usize,
) -> MatchResult {
    let mut player1_wins = 0;
    let mut player2_wins = 0;
    let mut draws = 0;
    let mut total_moves = 0;

    for game_num in 0..games_per_match {
        // Alternate who plays first
        let (black_score, white_score, moves) = if game_num % 2 == 0 {
            play_game(player1, player2, board_size)
        } else {
            let (w, b, m) = play_game(player2, player1, board_size);
            (b, w, m)
        };

        total_moves += moves;

        if black_score > white_score {
            if game_num % 2 == 0 {
                player1_wins += 1;
            } else {
                player2_wins += 1;
            }
        } else if white_score > black_score {
            if game_num % 2 == 0 {
                player2_wins += 1;
            } else {
                player1_wins += 1;
            }
        } else {
            draws += 1;
        }
    }

    MatchResult {
        player1_name: player1.name().to_string(),
        player2_name: player2.name().to_string(),
        player1_wins,
        player2_wins,
        draws,
        total_games: games_per_match as i32,
        avg_moves: total_moves as f64 / games_per_match as f64,
    }
}

fn main() {
    println!("=== AI League Tournament ===");
    println!("Board size: 5x5");
    println!("Games per match: 10 (5 as Black, 5 as White)\n");

    // Create output directory
    create_dir_all("league_results").unwrap();

    // Time limits to test (in milliseconds)
    let time_limits = vec![100, 200, 300, 500, 1000];

    // Create AI instances
    let mut ai_players: Vec<Box<dyn Player>> = Vec::new();
    let mut ai_names: Vec<String> = Vec::new();

    for &time_millis in &time_limits {
        ai_players.push(Box::new(MonteCarloAI::new_with_millis(time_millis)));
        ai_names.push(format!("MC_{:.1}s", time_millis as f64 / 1000.0));

        ai_players.push(Box::new(Mcts::new_with_millis(time_millis)));
        ai_names.push(format!("MCTS_{:.1}s", time_millis as f64 / 1000.0));
    }

    // Run league matches
    let mut results = Vec::new();
    let total_matches = (ai_players.len() * (ai_players.len() - 1)) / 2;
    let mut match_count = 0;

    for i in 0..ai_players.len() {
        for j in (i + 1)..ai_players.len() {
            match_count += 1;
            println!(
                "Match {}/{}: {} vs {}",
                match_count, total_matches, ai_names[i], ai_names[j]
            );

            let start = Instant::now();
            let result = run_match(&*ai_players[i], &*ai_players[j], 10, 5);
            let duration = start.elapsed();

            println!(
                "  Result: {}-{} (took {:.1}s)\n",
                result.player1_wins,
                result.player2_wins,
                duration.as_secs_f64()
            );

            results.push(result);
        }
    }

    // Generate markdown report
    let mut report = File::create("league_results/league_report.md").unwrap();

    writeln!(report, "# AI League Tournament Results\n").unwrap();
    writeln!(report, "## Tournament Settings").unwrap();
    writeln!(report, "- Board size: 5x5").unwrap();
    writeln!(report, "- Games per match: 10 (5 as Black, 5 as White)").unwrap();
    writeln!(
        report,
        "- Date: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M")
    )
    .unwrap();
    writeln!(report, "\n## AI Players").unwrap();
    writeln!(report, "| AI Type | Time Limit |").unwrap();
    writeln!(report, "|---------|------------|").unwrap();

    for name in &ai_names {
        let parts: Vec<&str> = name.split('_').collect();
        writeln!(report, "| {} | {} |", parts[0], parts[1]).unwrap();
    }

    writeln!(report, "\n## Match Results\n").unwrap();
    writeln!(
        report,
        "| Player 1 | Player 2 | P1 Wins | P2 Wins | Draws | Win Rate P1 | Avg Moves |"
    )
    .unwrap();
    writeln!(
        report,
        "|----------|----------|---------|---------|-------|-------------|-----------|"
    )
    .unwrap();

    for result in &results {
        let win_rate = result.player1_wins as f64 / result.total_games as f64 * 100.0;
        writeln!(
            report,
            "| {} | {} | {} | {} | {} | {:.1}% | {:.1} |",
            result.player1_name,
            result.player2_name,
            result.player1_wins,
            result.player2_wins,
            result.draws,
            win_rate,
            result.avg_moves
        )
        .unwrap();
    }

    // Calculate and display standings
    writeln!(report, "\n## Final Standings\n").unwrap();
    writeln!(
        report,
        "| Rank | AI Player | Total Wins | Total Games | Win Rate |"
    )
    .unwrap();
    writeln!(
        report,
        "|------|-----------|------------|-------------|----------|"
    )
    .unwrap();

    // Calculate total wins for each player
    let mut standings: Vec<(String, i32, i32)> = Vec::new();

    for (idx, name) in ai_names.iter().enumerate() {
        let mut total_wins = 0;
        let mut total_games = 0;

        for result in &results {
            if result.player1_name == *name {
                total_wins += result.player1_wins;
                total_games += result.total_games;
            } else if result.player2_name == *name {
                total_wins += result.player2_wins;
                total_games += result.total_games;
            }
        }

        standings.push((name.clone(), total_wins, total_games));
    }

    // Sort by wins
    standings.sort_by(|a, b| b.1.cmp(&a.1));

    for (rank, (name, wins, games)) in standings.iter().enumerate() {
        let win_rate = if *games > 0 {
            *wins as f64 / *games as f64 * 100.0
        } else {
            0.0
        };
        writeln!(
            report,
            "| {} | {} | {} | {} | {:.1}% |",
            rank + 1,
            name,
            wins,
            games,
            win_rate
        )
        .unwrap();
    }

    writeln!(report, "\n## Analysis\n").unwrap();
    writeln!(report, "### Performance by Time Limit").unwrap();
    writeln!(
        report,
        "- Comparison of how increased thinking time affects performance"
    )
    .unwrap();
    writeln!(report, "- MCTS vs Monte Carlo at each time limit").unwrap();

    writeln!(report, "\n### Key Observations").unwrap();
    writeln!(
        report,
        "1. **Time Impact**: How performance scales with thinking time"
    )
    .unwrap();
    writeln!(
        report,
        "2. **Algorithm Comparison**: MCTS vs Monte Carlo effectiveness"
    )
    .unwrap();
    writeln!(
        report,
        "3. **Game Length**: Average moves per game for different matchups"
    )
    .unwrap();

    println!("\nLeague tournament completed!");
    println!("Results saved to: league_results/league_report.md");
}
