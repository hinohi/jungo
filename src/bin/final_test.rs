use jungo::ai::{Mcts, RandomAI};
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

    let (black_stones, white_stones) = game.board.count_stones();
    let (black_captured, white_captured) = game.board.get_captured();

    let black_score = (black_stones + black_captured) as i32;
    let white_score = (white_stones + white_captured) as i32;

    (black_score, white_score)
}

fn main() {
    println!("=== Final MCTS Performance Test ===\n");

    println!("Testing MCTS (1s) vs Random - 10 games");
    let random_ai = RandomAI::new();
    let mcts_ai = Mcts::new(1);

    let mut mcts_wins = 0;
    let mut random_wins = 0;
    let mut draws = 0;

    let start_time = Instant::now();

    // 5 games with MCTS as Black
    for i in 0..5 {
        let (black_score, white_score) = run_game_silent(&mcts_ai, &random_ai, 5);
        if black_score > white_score {
            mcts_wins += 1;
            println!(
                "Game {}: MCTS wins ({}-{})",
                i + 1,
                black_score,
                white_score
            );
        } else if white_score > black_score {
            random_wins += 1;
            println!(
                "Game {}: Random wins ({}-{})",
                i + 1,
                black_score,
                white_score
            );
        } else {
            draws += 1;
            println!("Game {}: Draw ({}-{})", i + 1, black_score, white_score);
        }
    }

    // 5 games with MCTS as White
    for i in 0..5 {
        let (black_score, white_score) = run_game_silent(&random_ai, &mcts_ai, 5);
        if white_score > black_score {
            mcts_wins += 1;
            println!(
                "Game {}: MCTS wins ({}-{})",
                i + 6,
                black_score,
                white_score
            );
        } else if black_score > white_score {
            random_wins += 1;
            println!(
                "Game {}: Random wins ({}-{})",
                i + 6,
                black_score,
                white_score
            );
        } else {
            draws += 1;
            println!("Game {}: Draw ({}-{})", i + 6, black_score, white_score);
        }
    }

    println!("\nResults:");
    println!("MCTS: {} wins", mcts_wins);
    println!("Random: {} wins", random_wins);
    println!("Draws: {}", draws);
    println!("Time: {:.1}s", start_time.elapsed().as_secs_f64());

    if mcts_wins > random_wins {
        println!("\n✓ MCTS is performing better than Random!");
    } else {
        println!("\n✗ MCTS needs more improvement");
    }
}
