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

fn main() {
    println!("=== AI League Tournament (Quick Version) ===");
    println!("Board size: 5x5");
    println!("Games per match: 2 (1 as Black, 1 as White)\n");

    create_dir_all("league_results").unwrap();

    // Reduced time limits (only 3 settings)
    let time_limits = vec![100, 500, 1000];

    // Create AI instances
    let mut ai_list: Vec<(Box<dyn Player>, String)> = Vec::new();

    for &time_millis in &time_limits {
        ai_list.push((
            Box::new(MonteCarloAI::new_with_millis(time_millis)),
            format!("MC_{:.1}s", time_millis as f64 / 1000.0),
        ));

        ai_list.push((
            Box::new(Mcts::new_with_millis(time_millis)),
            format!("MCTS_{:.1}s", time_millis as f64 / 1000.0),
        ));
    }

    // Results storage
    let mut all_results = Vec::new();

    // Run matches
    let total_ais = ai_list.len();
    for i in 0..total_ais {
        for j in (i + 1)..total_ais {
            let (ai1, name1) = &ai_list[i];
            let (ai2, name2) = &ai_list[j];

            println!("Match: {} vs {}", name1, name2);
            let start = Instant::now();

            // Game 1: ai1 as Black
            let (b1, w1, _) = play_game(&**ai1, &**ai2, 5);

            // Game 2: ai2 as Black
            let (b2, w2, _) = play_game(&**ai2, &**ai1, 5);

            let ai1_wins = if b1 > w1 { 1 } else { 0 } + if w2 > b2 { 1 } else { 0 };
            let ai2_wins = if w1 > b1 { 1 } else { 0 } + if b2 > w2 { 1 } else { 0 };

            println!(
                "  Result: {}-{} (took {:.1}s)\n",
                ai1_wins,
                ai2_wins,
                start.elapsed().as_secs_f64()
            );

            all_results.push((name1.clone(), name2.clone(), ai1_wins, ai2_wins));
        }
    }

    // Generate report
    let mut report = File::create("league_results/quick_league_report.md").unwrap();

    writeln!(report, "# AI League Tournament Results (Quick Version)\n").unwrap();
    writeln!(report, "## Tournament Settings").unwrap();
    writeln!(report, "- Board size: 5x5").unwrap();
    writeln!(report, "- Games per match: 2 (1 as Black, 1 as White)").unwrap();
    writeln!(report, "- Time settings: 0.1s, 0.5s, 1.0s").unwrap();
    writeln!(
        report,
        "- Date: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M")
    )
    .unwrap();

    writeln!(report, "\n## Match Results\n").unwrap();
    writeln!(report, "| Player 1 | Player 2 | P1 Wins | P2 Wins |").unwrap();
    writeln!(report, "|----------|----------|---------|---------|").unwrap();

    for (name1, name2, w1, w2) in &all_results {
        writeln!(report, "| {} | {} | {} | {} |", name1, name2, w1, w2).unwrap();
    }

    // Calculate standings
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

    let mut standings: std::collections::HashMap<String, (i32, i32)> =
        std::collections::HashMap::new();
    for (name1, name2, w1, w2) in &all_results {
        let entry1 = standings.entry(name1.clone()).or_insert((0, 0));
        entry1.0 += *w1;
        entry1.1 += 2;

        let entry2 = standings.entry(name2.clone()).or_insert((0, 0));
        entry2.0 += *w2;
        entry2.1 += 2;
    }

    let mut standings_vec: Vec<_> = standings.into_iter().collect();
    standings_vec.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));

    for (rank, (name, (wins, games))) in standings_vec.iter().enumerate() {
        let win_rate = *wins as f64 / *games as f64 * 100.0;
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

    writeln!(report, "\n## Summary\n").unwrap();
    writeln!(report, "- Total matches played: {}", all_results.len()).unwrap();
    writeln!(
        report,
        "- Quick tournament format (2 games per match) to fit within time constraints"
    )
    .unwrap();

    println!("Tournament completed! Results saved to league_results/quick_league_report.md");
}
