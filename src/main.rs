mod ai;
mod board;
mod game;
mod player;
mod zobrist;

use crate::ai::{Mcts, MinimaxAI, MonteCarloAI, RandomAI};
use crate::game::Game;
use crate::player::{HumanPlayer, Player};
use std::io::{self, Write};

fn main() {
    println!("純碁 (Jungo) - A simple Go variant");
    println!();

    // Select board size
    let board_size = select_board_size();

    // Select game mode
    let (player1, player2): (Box<dyn Player>, Box<dyn Player>) = match select_game_mode() {
        1 => (Box::new(HumanPlayer::new()), select_ai_player()),
        2 => (select_ai_player(), Box::new(HumanPlayer::new())),
        3 => (select_ai_player(), select_ai_player()),
        _ => unreachable!(),
    };

    let mut game = Game::new(board_size);
    game.play(player1.as_ref(), player2.as_ref());
}

fn select_board_size() -> usize {
    loop {
        println!("Select board size:");
        println!("1. 5x5");
        println!("2. 7x7");
        println!("3. 9x9");
        println!("4. 13x13");
        println!("5. 19x19");
        print!("Enter your choice (1-5): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return 5,
            "2" => return 7,
            "3" => return 9,
            "4" => return 13,
            "5" => return 19,
            _ => println!("Invalid choice. Please try again.\n"),
        }
    }
}

fn select_game_mode() -> u8 {
    loop {
        println!("\nSelect game mode:");
        println!("1. Human vs AI");
        println!("2. AI vs Human");
        println!("3. AI vs AI");
        print!("Enter your choice (1-3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return 1,
            "2" => return 2,
            "3" => return 3,
            _ => println!("Invalid choice. Please try again.\n"),
        }
    }
}

fn select_ai_player() -> Box<dyn Player> {
    loop {
        println!("\nSelect AI type:");
        println!("1. Random AI");
        println!("2. Minimax AI (depth 3)");
        println!("3. Minimax AI (depth 5)");
        println!("4. Monte Carlo AI (1 second)");
        println!("5. Monte Carlo AI (3 seconds)");
        println!("6. MCTS AI (1 second)");
        println!("7. MCTS AI (3 seconds)");
        print!("Enter your choice (1-7): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return Box::new(RandomAI::new()),
            "2" => return Box::new(MinimaxAI::new(3)),
            "3" => return Box::new(MinimaxAI::new(5)),
            "4" => return Box::new(MonteCarloAI::new(1)),
            "5" => return Box::new(MonteCarloAI::new(3)),
            "6" => return Box::new(Mcts::new(1)),
            "7" => return Box::new(Mcts::new(3)),
            _ => println!("Invalid choice. Please try again.\n"),
        }
    }
}
