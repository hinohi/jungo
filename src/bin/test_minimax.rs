use jungo::ai::{MinimaxAI, RandomAI};
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
                // Check if the move is valid
                if !game.board.is_valid_move(x, y, game.current_turn) {
                    continue;
                }

                // Clone board to test the move
                let mut test_board = game.board.clone();
                if test_board.place_stone(x, y, game.current_turn).is_ok() {
                    let new_hash = test_board.get_hash();

                    // Check Ko rule: see if this board state occurred 2 moves ago
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

fn run_tournament(num_games: u32) {
    println!("=== Minimax vs Random AI Tournament ===");
    println!("Board size: 7x7");
    println!("Minimax depth: 2");
    println!("Number of games: {}\n", num_games);

    let random_ai = RandomAI::new();
    let minimax_ai = MinimaxAI::new(2); // Depth 2 for reasonable speed

    let mut minimax_as_black_wins = 0;
    let mut minimax_as_white_wins = 0;
    let mut total_games = 0;

    let start_time = Instant::now();

    // Half games with Minimax as Black
    println!("Running {} games with Minimax as Black...", num_games / 2);
    for i in 0..(num_games / 2) {
        if i % 10 == 0 && i > 0 {
            print!("Progress: {}/{} games...\r", i, num_games / 2);
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        }

        let (black_score, white_score) = run_game_silent(&minimax_ai, &random_ai, 7);
        if black_score > white_score {
            minimax_as_black_wins += 1;
        }
        total_games += 1;
    }
    println!("\nCompleted {} games with Minimax as Black", num_games / 2);

    // Half games with Minimax as White
    println!("\nRunning {} games with Minimax as White...", num_games / 2);
    for i in 0..(num_games / 2) {
        if i % 10 == 0 && i > 0 {
            print!("Progress: {}/{} games...\r", i, num_games / 2);
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        }

        let (black_score, white_score) = run_game_silent(&random_ai, &minimax_ai, 7);
        if white_score > black_score {
            minimax_as_white_wins += 1;
        }
        total_games += 1;
    }
    println!("\nCompleted {} games with Minimax as White", num_games / 2);

    let total_minimax_wins = minimax_as_black_wins + minimax_as_white_wins;
    let win_rate = (total_minimax_wins as f64 / total_games as f64) * 100.0;

    println!("\n=== Tournament Results ===");
    println!("Total games: {}", total_games);
    println!(
        "Minimax wins as Black: {} / {}",
        minimax_as_black_wins,
        num_games / 2
    );
    println!(
        "Minimax wins as White: {} / {}",
        minimax_as_white_wins,
        num_games / 2
    );
    println!(
        "Total Minimax wins: {} ({:.1}%)",
        total_minimax_wins, win_rate
    );
    println!(
        "Total Random wins: {} ({:.1}%)",
        total_games - total_minimax_wins,
        100.0 - win_rate
    );
    println!("Time elapsed: {:.2}s", start_time.elapsed().as_secs_f64());
}

fn main() {
    run_tournament(100);
}
