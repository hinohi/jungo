use jungo::ai::{Mcts, MonteCarloAI, RandomAI};
use jungo::board::Stone;
use jungo::game::Game;
use jungo::player::Player;
use std::fs::{create_dir_all, File};
use std::io::Write;

fn play_game(player1: &dyn Player, player2: &dyn Player) -> i32 {
    let mut game = Game::new(5); // 5x5 board for faster games
    let mut timeout = 0;

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

        timeout += 1;
        if timeout > 100 {
            // Prevent infinite games
            break;
        }
    }

    let (black_stones, white_stones) = game.board.count_stones();
    let (black_captured, white_captured) = game.board.get_captured();
    (black_stones + black_captured) as i32 - (white_stones + white_captured) as i32
}

fn main() {
    println!("=== MCTS Mini Test (5x5 board) ===\n");

    create_dir_all("mcts_results").unwrap();
    let mut file = File::create("mcts_results/mini_test.txt").unwrap();

    writeln!(file, "MCTS Strength Test - 5x5 Board").unwrap();
    writeln!(file, "================================\n").unwrap();

    // Test 1: Single game examples
    println!("Sample games:");

    let mcts1 = Mcts::new(1);
    let mcts3 = Mcts::new(3);
    let random = RandomAI::new();

    print!("MCTS 1s vs Random: ");
    let score1 = play_game(&mcts1, &random);
    println!("{:+}", score1);

    print!("MCTS 3s vs Random: ");
    let score2 = play_game(&mcts3, &random);
    println!("{:+}", score2);

    print!("MCTS 3s vs MCTS 1s: ");
    let score3 = play_game(&mcts3, &mcts1);
    println!("{:+}", score3);

    // Test 2: Multiple games
    println!("\nMultiple game test (2 games each):");
    writeln!(file, "Matchup Results (2 games each):").unwrap();
    writeln!(file, "================================").unwrap();

    // MCTS 1s vs Random
    print!("MCTS 1s vs Random: ");
    let mut wins = 0;
    for i in 0..2 {
        let score = if i % 2 == 0 {
            play_game(&mcts1, &random)
        } else {
            -play_game(&random, &mcts1)
        };
        if score > 0 {
            wins += 1;
            print!("W ");
        } else {
            print!("L ");
        }
    }
    println!("=> {}/2", wins);
    writeln!(file, "MCTS 1s vs Random: {}/2 wins", wins).unwrap();

    // MCTS 3s vs Random
    print!("MCTS 3s vs Random: ");
    wins = 0;
    for i in 0..2 {
        let score = if i % 2 == 0 {
            play_game(&mcts3, &random)
        } else {
            -play_game(&random, &mcts3)
        };
        if score > 0 {
            wins += 1;
            print!("W ");
        } else {
            print!("L ");
        }
    }
    println!("=> {}/2", wins);
    writeln!(file, "MCTS 3s vs Random: {}/2 wins", wins).unwrap();

    // MCTS 3s vs MCTS 1s
    print!("MCTS 3s vs MCTS 1s: ");
    wins = 0;
    for i in 0..2 {
        let score = if i % 2 == 0 {
            play_game(&mcts3, &mcts1)
        } else {
            -play_game(&mcts1, &mcts3)
        };
        if score > 0 {
            wins += 1;
            print!("W ");
        } else {
            print!("L ");
        }
    }
    println!("=> {}/2", wins);
    writeln!(file, "MCTS 3s vs MCTS 1s: {}/2 wins", wins).unwrap();

    // Test with Monte Carlo
    let mc1 = MonteCarloAI::new(1);
    print!("MCTS 1s vs MC 1s: ");
    wins = 0;
    for i in 0..2 {
        let score = if i % 2 == 0 {
            play_game(&mcts1, &mc1)
        } else {
            -play_game(&mc1, &mcts1)
        };
        if score > 0 {
            wins += 1;
            print!("W ");
        } else {
            print!("L ");
        }
    }
    println!("=> {}/2", wins);
    writeln!(file, "MCTS 1s vs MC 1s: {}/2 wins", wins).unwrap();

    writeln!(file, "\nConclusion:").unwrap();
    writeln!(file, "More time generally improves MCTS performance.").unwrap();
    writeln!(file, "Even 1s MCTS should beat Random AI consistently.").unwrap();

    println!("\n=== Results saved to mcts_results/mini_test.txt ===");
}
