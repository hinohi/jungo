use jungo::ai::{Mcts, MonteCarloAI};
use jungo::board::Stone;
use jungo::game::Game;
use jungo::player::Player;
use std::env;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;

fn play_game(player1: &dyn Player, player2: &dyn Player) -> (i32, i32) {
    let mut game = Game::new(5);
    let mut move_count = 0;
    let mercy_threshold = 12; // Early termination threshold

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

        // Check mercy rule
        if move_count > 20 && move_count % 5 == 0 {
            let (black_stones, white_stones) = game.board.count_stones();
            let (black_captured, white_captured) = game.board.get_captured();
            let black_score = (black_stones + black_captured) as i32;
            let white_score = (white_stones + white_captured) as i32;

            if (black_score - white_score).abs() > mercy_threshold {
                break; // Mercy rule
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
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Usage: {} <ai1_type> <ai1_time_ms> <ai2_type> <ai2_time_ms>",
            args[0]
        );
        eprintln!("AI types: MC or MCTS");
        eprintln!("Example: {} MC 500 MCTS 1000", args[0]);
        std::process::exit(1);
    }

    let ai1_type = &args[1];
    let ai1_time: u64 = args[2].parse().expect("Invalid time for AI1");
    let ai2_type = &args[3];
    let ai2_time: u64 = args[4].parse().expect("Invalid time for AI2");

    // Create AI instances
    let ai1: Box<dyn Player> = match ai1_type.as_str() {
        "MC" => Box::new(MonteCarloAI::new_with_millis(ai1_time)),
        "MCTS" => Box::new(Mcts::new_with_millis(ai1_time)),
        _ => panic!("Invalid AI type for AI1"),
    };

    let ai2: Box<dyn Player> = match ai2_type.as_str() {
        "MC" => Box::new(MonteCarloAI::new_with_millis(ai2_time)),
        "MCTS" => Box::new(Mcts::new_with_millis(ai2_time)),
        _ => panic!("Invalid AI type for AI2"),
    };

    let ai1_name = format!("{}_{:.1}s", ai1_type, ai1_time as f64 / 1000.0);
    let ai2_name = format!("{}_{:.1}s", ai2_type, ai2_time as f64 / 1000.0);

    println!("Running match: {} vs {}", ai1_name, ai2_name);

    // Play 10 games
    let mut ai1_wins = 0;
    let mut ai2_wins = 0;

    for game_num in 0..10 {
        print!("Game {}/10... ", game_num + 1);
        std::io::stdout().flush().unwrap();

        let (black_score, white_score) = if game_num % 2 == 0 {
            play_game(&*ai1, &*ai2)
        } else {
            let (w, b) = play_game(&*ai2, &*ai1);
            (b, w)
        };

        if black_score > white_score {
            if game_num % 2 == 0 {
                ai1_wins += 1;
            } else {
                ai2_wins += 1;
            }
            println!("Black wins");
        } else {
            if game_num % 2 == 0 {
                ai2_wins += 1;
            } else {
                ai1_wins += 1;
            }
            println!("White wins");
        }
    }

    println!(
        "\nFinal result: {} {}-{} {}",
        ai1_name, ai1_wins, ai2_wins, ai2_name
    );

    // Save result to file
    create_dir_all("league_results").unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("league_results/match_results.csv")
        .unwrap();

    writeln!(file, "{},{},{},{}", ai1_name, ai2_name, ai1_wins, ai2_wins).unwrap();
}
