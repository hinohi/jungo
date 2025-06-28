use jungo::ai::{MonteCarloAI, RandomAI};
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
    println!("=== Monte Carlo vs Random AI Tournament ===");
    println!("Board size: 5x5");
    println!("Monte Carlo time limit: 1 second per move");
    println!("Number of games: 10\n");

    let random_ai = RandomAI::new();
    let mc_ai = MonteCarloAI::new(1); // 1 second per move

    let mut mc_wins = 0;
    let mut random_wins = 0;
    let mut draws = 0;

    let start_time = Instant::now();

    // 5 games with Monte Carlo as Black
    println!("Running 5 games with Monte Carlo as Black...");
    for i in 0..5 {
        print!("Game {}... ", i + 1);
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let game_start = Instant::now();
        let (black_score, white_score) = run_game_silent(&mc_ai, &random_ai, 5);
        let game_time = game_start.elapsed();

        if black_score > white_score {
            mc_wins += 1;
            println!(
                "Monte Carlo wins! ({}:{}) [{:.1}s]",
                black_score,
                white_score,
                game_time.as_secs_f64()
            );
        } else if white_score > black_score {
            random_wins += 1;
            println!(
                "Random wins. ({}:{}) [{:.1}s]",
                black_score,
                white_score,
                game_time.as_secs_f64()
            );
        } else {
            draws += 1;
            println!(
                "Draw. ({}:{}) [{:.1}s]",
                black_score,
                white_score,
                game_time.as_secs_f64()
            );
        }
    }

    // 5 games with Monte Carlo as White
    println!("\nRunning 5 games with Monte Carlo as White...");
    for i in 0..5 {
        print!("Game {}... ", i + 1);
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let game_start = Instant::now();
        let (black_score, white_score) = run_game_silent(&random_ai, &mc_ai, 5);
        let game_time = game_start.elapsed();

        if white_score > black_score {
            mc_wins += 1;
            println!(
                "Monte Carlo wins! ({}:{}) [{:.1}s]",
                black_score,
                white_score,
                game_time.as_secs_f64()
            );
        } else if black_score > white_score {
            random_wins += 1;
            println!(
                "Random wins. ({}:{}) [{:.1}s]",
                black_score,
                white_score,
                game_time.as_secs_f64()
            );
        } else {
            draws += 1;
            println!(
                "Draw. ({}:{}) [{:.1}s]",
                black_score,
                white_score,
                game_time.as_secs_f64()
            );
        }
    }

    let total_games = mc_wins + random_wins + draws;
    let win_rate = (mc_wins as f64 / total_games as f64) * 100.0;

    println!("\n=== Tournament Results ===");
    println!("Total games: {}", total_games);
    println!("Monte Carlo wins: {} ({:.1}%)", mc_wins, win_rate);
    println!(
        "Random wins: {} ({:.1}%)",
        random_wins,
        (random_wins as f64 / total_games as f64) * 100.0
    );
    println!(
        "Draws: {} ({:.1}%)",
        draws,
        (draws as f64 / total_games as f64) * 100.0
    );
    println!("Total time: {:.1}s", start_time.elapsed().as_secs_f64());
}
