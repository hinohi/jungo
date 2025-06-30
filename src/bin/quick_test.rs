use jungo::ai::{Mcts, MonteCarloAI, RandomAI};
use jungo::board::Stone;
use jungo::game::Game;
use jungo::player::Player;

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

    let (black_stones, white_stones) = game.board.count_stones();
    let (black_captured, white_captured) = game.board.get_captured();

    let black_score = (black_stones + black_captured) as i32;
    let white_score = (white_stones + white_captured) as i32;

    (black_score, white_score)
}

fn main() {
    println!("=== Quick Performance Test ===\n");

    // Test 1: MCTS vs Random (10 games)
    println!("Test 1: MCTS (0.5s) vs Random");
    let random_ai = RandomAI::new();
    let mcts_ai = Mcts::new(1); // Using 1s but will time with 0.5s games

    let mut mcts_wins = 0;
    let mut random_wins = 0;

    for i in 0..10 {
        let (black_score, white_score) = if i < 5 {
            run_game_silent(&mcts_ai, &random_ai, 5)
        } else {
            run_game_silent(&random_ai, &mcts_ai, 5)
        };

        if i < 5 {
            if black_score > white_score {
                mcts_wins += 1;
            } else {
                random_wins += 1;
            }
        } else if white_score > black_score {
            mcts_wins += 1;
        } else {
            random_wins += 1;
        }

        print!(".");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    println!("\nMCTS: {} wins, Random: {} wins", mcts_wins, random_wins);

    // Test 2: MCTS vs MC (4 games)
    println!("\nTest 2: MCTS vs Monte Carlo (1s each)");
    let mc_ai = MonteCarloAI::new(1);

    let mut mcts_wins2 = 0;
    let mut mc_wins = 0;

    for i in 0..4 {
        let (black_score, white_score) = if i < 2 {
            run_game_silent(&mcts_ai, &mc_ai, 5)
        } else {
            run_game_silent(&mc_ai, &mcts_ai, 5)
        };

        if i < 2 {
            if black_score > white_score {
                mcts_wins2 += 1;
            } else {
                mc_wins += 1;
            }
        } else if white_score > black_score {
            mcts_wins2 += 1;
        } else {
            mc_wins += 1;
        }

        print!(".");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    println!("\nMCTS: {} wins, MC: {} wins", mcts_wins2, mc_wins);
}
