use jungo::ai::{Mcts, MonteCarloAI, RandomAI};
use jungo::board::Stone;
use jungo::game::Game;
use jungo::player::Player;
use std::time::Instant;

fn run_game_silent(player1: &dyn Player, player2: &dyn Player, board_size: usize) -> (i32, i32) {
    let mut game = Game::new(board_size);

    loop {
        let current_player: &dyn Player = match game.current_turn {
            Stone::Black => player1,
            Stone::White => player2,
        };

        match current_player.get_move(&game.board, game.current_turn) {
            Some((x, y)) => {
                if let Some(ref prev_board) = game.previous_board {
                    if game
                        .board
                        .is_valid_move_with_ko(x, y, game.current_turn, prev_board)
                    {
                        let board_before_move = game.board.clone();

                        if game.board.place_stone(x, y, game.current_turn).is_ok() {
                            game.consecutive_passes = 0;
                            game.previous_board = Some(board_before_move);
                        }
                    }
                } else {
                    let board_before_move = game.board.clone();

                    if game.board.place_stone(x, y, game.current_turn).is_ok() {
                        game.consecutive_passes = 0;
                        game.previous_board = Some(board_before_move);
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

    // Calculate final scores
    let (black_stones, white_stones) = game.board.count_stones();
    let (black_captured, white_captured) = game.board.get_captured();

    let black_score = (black_stones + black_captured) as i32;
    let white_score = (white_stones + white_captured) as i32;

    (black_score, white_score)
}

fn main() {
    println!("=== MCTS Debug Analysis ===\n");

    // Quick test with different time limits
    let time_limits = vec![1];

    for time_limit in time_limits {
        println!("MCTS ({}s) vs Random - 20 games", time_limit);
        let random_ai = RandomAI::new();
        let mcts_ai = Mcts::new(time_limit);

        let mut mcts_wins = 0;
        let mut random_wins = 0;
        let mut draws = 0;

        let start_time = Instant::now();

        // 10 games with MCTS as Black
        for i in 0..10 {
            print!("Game {}: ", i + 1);
            let (black_score, white_score) = run_game_silent(&mcts_ai, &random_ai, 5);
            if black_score > white_score {
                mcts_wins += 1;
                println!("MCTS wins ({}-{})", black_score, white_score);
            } else if white_score > black_score {
                random_wins += 1;
                println!("Random wins ({}-{})", black_score, white_score);
            } else {
                draws += 1;
                println!("Draw ({}-{})", black_score, white_score);
            }
        }

        // 10 games with MCTS as White
        for i in 0..10 {
            print!("Game {}: ", i + 11);
            let (black_score, white_score) = run_game_silent(&random_ai, &mcts_ai, 5);
            if white_score > black_score {
                mcts_wins += 1;
                println!("MCTS wins ({}-{})", black_score, white_score);
            } else if black_score > white_score {
                random_wins += 1;
                println!("Random wins ({}-{})", black_score, white_score);
            } else {
                draws += 1;
                println!("Draw ({}-{})", black_score, white_score);
            }
        }

        let total_games = mcts_wins + random_wins + draws;
        let win_rate = (mcts_wins as f64 / total_games as f64) * 100.0;

        println!("\nResults:");
        println!("  MCTS wins: {} ({:.1}%)", mcts_wins, win_rate);
        println!(
            "  Random wins: {} ({:.1}%)",
            random_wins,
            (random_wins as f64 / total_games as f64) * 100.0
        );
        println!(
            "  Draws: {} ({:.1}%)",
            draws,
            (draws as f64 / total_games as f64) * 100.0
        );
        println!("  Time: {:.1}s\n", start_time.elapsed().as_secs_f64());
    }

    // Compare with Monte Carlo
    println!("\nMCTS (1s) vs Monte Carlo (1s) - 10 games");
    let mc_ai = MonteCarloAI::new(1);
    let mcts_ai = Mcts::new(1);

    let mut mcts_wins = 0;
    let mut mc_wins = 0;
    let mut draws = 0;

    let start_time = Instant::now();

    for i in 0..5 {
        print!("Game {}: ", i + 1);
        let (black_score, white_score) = run_game_silent(&mcts_ai, &mc_ai, 5);
        if black_score > white_score {
            mcts_wins += 1;
            println!("MCTS wins ({}-{})", black_score, white_score);
        } else if white_score > black_score {
            mc_wins += 1;
            println!("MC wins ({}-{})", black_score, white_score);
        } else {
            draws += 1;
            println!("Draw ({}-{})", black_score, white_score);
        }
    }

    for i in 0..5 {
        print!("Game {}: ", i + 6);
        let (black_score, white_score) = run_game_silent(&mc_ai, &mcts_ai, 5);
        if white_score > black_score {
            mcts_wins += 1;
            println!("MCTS wins ({}-{})", black_score, white_score);
        } else if black_score > white_score {
            mc_wins += 1;
            println!("MC wins ({}-{})", black_score, white_score);
        } else {
            draws += 1;
            println!("Draw ({}-{})", black_score, white_score);
        }
    }

    let total_games = mcts_wins + mc_wins + draws;
    let win_rate = (mcts_wins as f64 / total_games as f64) * 100.0;

    println!("\nResults:");
    println!("  MCTS wins: {} ({:.1}%)", mcts_wins, win_rate);
    println!(
        "  MC wins: {} ({:.1}%)",
        mc_wins,
        (mc_wins as f64 / total_games as f64) * 100.0
    );
    println!(
        "  Draws: {} ({:.1}%)",
        draws,
        (draws as f64 / total_games as f64) * 100.0
    );
    println!("  Time: {:.1}s", start_time.elapsed().as_secs_f64());
}
