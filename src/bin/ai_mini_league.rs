use jungo::ai::{Mcts, MonteCarloAI};
use jungo::board::Stone;
use jungo::game::Game;
use jungo::player::Player;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::time::Instant;

fn play_single_game(player1: &dyn Player, player2: &dyn Player) -> (i32, i32) {
    let mut game = Game::new(5);
    let mut move_count = 0;
    let max_moves = 50; // Limit moves to speed up

    loop {
        let current_player: &dyn Player = match game.current_turn {
            Stone::Black => player1,
            Stone::White => player2,
        };

        match current_player.get_move(&game.board, game.current_turn) {
            Some((x, y)) => {
                if game.board.is_valid_move(x, y, game.current_turn) {
                    let mut test_board = game.board.clone();
                    if test_board.place_stone(x, y, game.current_turn).is_ok() {
                        let new_hash = test_board.get_hash();
                        let history_len = game.board_history.len();
                        if (history_len < 2 || game.board_history[history_len - 2] != new_hash)
                            && game.board.place_stone(x, y, game.current_turn).is_ok()
                        {
                            move_count += 1;
                            game.consecutive_passes = 0;
                            game.board_history.push(game.board.get_hash());
                        }
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

        if move_count >= max_moves {
            break;
        }
    }

    let (black_stones, white_stones) = game.board.count_stones();
    let (black_captured, white_captured) = game.board.get_captured();
    (
        (black_stones + black_captured) as i32,
        (white_stones + white_captured) as i32,
    )
}

fn main() {
    println!("=== AI Mini League ===");
    println!("Testing 0.5s and 1.0s versions only");
    println!("1 game per match\n");

    create_dir_all("league_results").unwrap();

    // Only test 0.5s and 1.0s
    let mc_05 = MonteCarloAI::new_with_millis(500);
    let mc_10 = MonteCarloAI::new_with_millis(1000);
    let mcts_05 = Mcts::new_with_millis(500);
    let mcts_10 = Mcts::new_with_millis(1000);

    let mut results = Vec::new();

    // MC 0.5s vs MCTS 0.5s
    println!("Match 1: MC_0.5s vs MCTS_0.5s");
    let start = Instant::now();
    let (b1, w1) = play_single_game(&mc_05, &mcts_05);
    let winner1 = if b1 > w1 { "MC_0.5s" } else { "MCTS_0.5s" };
    println!(
        "  Winner: {} (Score: {}-{}, Time: {:.1}s)\n",
        winner1,
        b1,
        w1,
        start.elapsed().as_secs_f64()
    );
    results.push(("MC_0.5s", "MCTS_0.5s", b1, w1));

    // MC 0.5s vs MCTS 1.0s
    println!("Match 2: MC_0.5s vs MCTS_1.0s");
    let start = Instant::now();
    let (b2, w2) = play_single_game(&mc_05, &mcts_10);
    let winner2 = if b2 > w2 { "MC_0.5s" } else { "MCTS_1.0s" };
    println!(
        "  Winner: {} (Score: {}-{}, Time: {:.1}s)\n",
        winner2,
        b2,
        w2,
        start.elapsed().as_secs_f64()
    );
    results.push(("MC_0.5s", "MCTS_1.0s", b2, w2));

    // MC 1.0s vs MCTS 0.5s
    println!("Match 3: MC_1.0s vs MCTS_0.5s");
    let start = Instant::now();
    let (b3, w3) = play_single_game(&mc_10, &mcts_05);
    let winner3 = if b3 > w3 { "MC_1.0s" } else { "MCTS_0.5s" };
    println!(
        "  Winner: {} (Score: {}-{}, Time: {:.1}s)\n",
        winner3,
        b3,
        w3,
        start.elapsed().as_secs_f64()
    );
    results.push(("MC_1.0s", "MCTS_0.5s", b3, w3));

    // MC 1.0s vs MCTS 1.0s
    println!("Match 4: MC_1.0s vs MCTS_1.0s");
    let start = Instant::now();
    let (b4, w4) = play_single_game(&mc_10, &mcts_10);
    let winner4 = if b4 > w4 { "MC_1.0s" } else { "MCTS_1.0s" };
    println!(
        "  Winner: {} (Score: {}-{}, Time: {:.1}s)\n",
        winner4,
        b4,
        w4,
        start.elapsed().as_secs_f64()
    );
    results.push(("MC_1.0s", "MCTS_1.0s", b4, w4));

    // MC 0.5s vs MC 1.0s
    println!("Match 5: MC_0.5s vs MC_1.0s");
    let start = Instant::now();
    let (b5, w5) = play_single_game(&mc_05, &mc_10);
    let winner5 = if b5 > w5 { "MC_0.5s" } else { "MC_1.0s" };
    println!(
        "  Winner: {} (Score: {}-{}, Time: {:.1}s)\n",
        winner5,
        b5,
        w5,
        start.elapsed().as_secs_f64()
    );
    results.push(("MC_0.5s", "MC_1.0s", b5, w5));

    // MCTS 0.5s vs MCTS 1.0s
    println!("Match 6: MCTS_0.5s vs MCTS_1.0s");
    let start = Instant::now();
    let (b6, w6) = play_single_game(&mcts_05, &mcts_10);
    let winner6 = if b6 > w6 { "MCTS_0.5s" } else { "MCTS_1.0s" };
    println!(
        "  Winner: {} (Score: {}-{}, Time: {:.1}s)\n",
        winner6,
        b6,
        w6,
        start.elapsed().as_secs_f64()
    );
    results.push(("MCTS_0.5s", "MCTS_1.0s", b6, w6));

    // Write results
    let mut report = File::create("league_results/mini_league_report.md").unwrap();

    writeln!(report, "# AI Mini League Results\n").unwrap();
    writeln!(report, "## Settings").unwrap();
    writeln!(report, "- Board size: 5x5").unwrap();
    writeln!(report, "- Games per match: 1 (Black player listed first)").unwrap();
    writeln!(report, "- Time settings tested: 0.5s, 1.0s").unwrap();
    writeln!(
        report,
        "- Date: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M")
    )
    .unwrap();

    writeln!(report, "## Match Results\n").unwrap();
    writeln!(
        report,
        "| Black Player | White Player | Black Score | White Score | Winner |"
    )
    .unwrap();
    writeln!(
        report,
        "|--------------|--------------|-------------|-------------|---------|"
    )
    .unwrap();

    for (black, white, b_score, w_score) in &results {
        let winner = if b_score > w_score { black } else { white };
        writeln!(
            report,
            "| {} | {} | {} | {} | {} |",
            black, white, b_score, w_score, winner
        )
        .unwrap();
    }

    writeln!(report, "\n## Key Observations\n").unwrap();
    writeln!(
        report,
        "1. **MCTS vs Monte Carlo**: Compare performance between algorithms"
    )
    .unwrap();
    writeln!(
        report,
        "2. **Time Impact**: How 0.5s vs 1.0s affects each algorithm"
    )
    .unwrap();
    writeln!(
        report,
        "3. **First Move Advantage**: Black player performance"
    )
    .unwrap();

    println!("\nResults saved to league_results/mini_league_report.md");
}
