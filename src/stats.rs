use crate::ai::RandomAI;
use crate::board::Stone;
use crate::game::Game;
use crate::player::Player;
use std::time::Instant;

pub struct GameStats {
    pub black_wins: u32,
    pub white_wins: u32,
    pub draws: u32,
    pub total_black_score: i32,
    pub total_white_score: i32,
    pub total_moves: u32,
    pub total_duration: std::time::Duration,
}

impl Default for GameStats {
    fn default() -> Self {
        GameStats {
            black_wins: 0,
            white_wins: 0,
            draws: 0,
            total_black_score: 0,
            total_white_score: 0,
            total_moves: 0,
            total_duration: std::time::Duration::new(0, 0),
        }
    }
}

impl GameStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print_summary(&self, total_games: u32, board_size: usize) {
        println!(
            "\n=== Game Statistics for {}x{} Board ===",
            board_size, board_size
        );
        println!("Total games played: {}", total_games);
        println!("\nWin Statistics:");
        println!(
            "Black wins: {} ({:.1}%)",
            self.black_wins,
            (self.black_wins as f64 / total_games as f64) * 100.0
        );
        println!(
            "White wins: {} ({:.1}%)",
            self.white_wins,
            (self.white_wins as f64 / total_games as f64) * 100.0
        );
        println!(
            "Draws: {} ({:.1}%)",
            self.draws,
            (self.draws as f64 / total_games as f64) * 100.0
        );

        println!("\nScore Statistics:");
        println!(
            "Average Black score: {:.2}",
            self.total_black_score as f64 / total_games as f64
        );
        println!(
            "Average White score: {:.2}",
            self.total_white_score as f64 / total_games as f64
        );
        println!(
            "Average score difference: {:.2}",
            (self.total_black_score - self.total_white_score) as f64 / total_games as f64
        );

        println!("\nGame Statistics:");
        println!(
            "Average moves per game: {:.2}",
            self.total_moves as f64 / total_games as f64
        );
        println!(
            "Average game duration: {:.2}ms",
            self.total_duration.as_millis() as f64 / total_games as f64
        );
        println!("Total time: {:.2}s", self.total_duration.as_secs_f64());
    }
}

pub fn run_game_silent(board_size: usize) -> (i32, i32, u32) {
    let mut game = Game::new(board_size);
    let player1 = RandomAI::new();
    let player2 = RandomAI::new();
    let mut move_count = 0;

    loop {
        let current_player: &dyn Player = match game.current_turn {
            Stone::Black => &player1,
            Stone::White => &player2,
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
                            move_count += 1;
                        }
                    }
                } else {
                    let board_before_move = game.board.clone();

                    if game.board.place_stone(x, y, game.current_turn).is_ok() {
                        game.consecutive_passes = 0;
                        game.previous_board = Some(board_before_move);
                        move_count += 1;
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

    (black_score, white_score, move_count)
}

pub fn run_statistics(board_size: usize, num_games: u32) -> GameStats {
    let mut stats = GameStats::new();
    let _start_time = Instant::now();

    println!(
        "Running {} games on {}x{} board...",
        num_games, board_size, board_size
    );

    for i in 0..num_games {
        if i % 1000 == 0 && i > 0 {
            print!(
                "Progress: {}/{} games ({:.1}%)...\r",
                i,
                num_games,
                (i as f64 / num_games as f64) * 100.0
            );
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        }

        let game_start = Instant::now();
        let (black_score, white_score, moves) = run_game_silent(board_size);
        let game_duration = game_start.elapsed();

        stats.total_black_score += black_score;
        stats.total_white_score += white_score;
        stats.total_moves += moves;
        stats.total_duration += game_duration;

        if black_score > white_score {
            stats.black_wins += 1;
        } else if white_score > black_score {
            stats.white_wins += 1;
        } else {
            stats.draws += 1;
        }
    }

    println!("\nCompleted {} games!", num_games);
    stats
}
