use crate::ai::RandomAI;
use crate::board::{Board, Stone};
use crate::player::Player;
use std::time::{Duration, Instant};

pub struct MonteCarloAI {
    name: String,
    time_limit: Duration,
}

impl MonteCarloAI {
    pub fn new(time_seconds: u64) -> Self {
        MonteCarloAI {
            name: format!("Monte Carlo AI ({}s)", time_seconds),
            time_limit: Duration::from_secs(time_seconds),
        }
    }

    fn simulate_game(&self, board: &Board, stone: Stone, first_move: (usize, usize)) -> f64 {
        // Create a new board with the same state including captured stones
        let mut sim_board = board.clone();

        // Apply the first move
        if sim_board
            .place_stone(first_move.0, first_move.1, stone)
            .is_err()
        {
            // Invalid move, return loss
            return 0.0;
        }

        let mut current_turn = stone.opposite();
        let mut consecutive_passes = 0;

        // Create two RandomAI players
        let random1 = RandomAI::new();
        let random2 = RandomAI::new();

        // Play out the game with a maximum number of moves to prevent long games
        let mut moves = 0;
        let max_moves = board.size() * board.size() * 2;

        loop {
            let current_player: &dyn Player = match current_turn {
                s if s == stone => &random1,
                _ => &random2,
            };

            match current_player.get_move(&sim_board, current_turn) {
                Some((x, y)) => {
                    // In simulation, we don't track Ko rule for performance
                    if sim_board.place_stone(x, y, current_turn).is_ok() {
                        consecutive_passes = 0;
                    }
                }
                None => {
                    consecutive_passes += 1;
                    if consecutive_passes >= 2 {
                        break;
                    }
                }
            }

            current_turn = current_turn.opposite();

            moves += 1;
            if moves >= max_moves {
                break;
            }
        }

        // Evaluate final position
        let (black_stones, white_stones) = sim_board.count_stones();
        let (black_captured, white_captured) = sim_board.get_captured();

        let black_score = (black_stones + black_captured) as i32;
        let white_score = (white_stones + white_captured) as i32;

        // Return win (1.0) or loss (0.0) from perspective of the original stone
        match stone {
            Stone::Black => {
                if black_score > white_score {
                    1.0
                } else {
                    0.0
                }
            }
            Stone::White => {
                if white_score > black_score {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
}

impl Player for MonteCarloAI {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&self, board: &Board, stone: Stone) -> Option<(usize, usize)> {
        let mut valid_moves = Vec::new();
        let mut non_eye_moves = Vec::new();

        for y in 0..board.size() {
            for x in 0..board.size() {
                if board.is_valid_move(x, y, stone) {
                    valid_moves.push((x, y));
                    if !board.is_eye(x, y, stone) {
                        non_eye_moves.push((x, y));
                    }
                }
            }
        }

        // Count total eyes for our color
        let total_eyes = board.count_eyes_for_color(stone);

        // If we have 2 or fewer eyes, only consider non-eye moves
        if total_eyes <= 2 && !non_eye_moves.is_empty() {
            valid_moves = non_eye_moves;
        } else if total_eyes <= 2 && non_eye_moves.is_empty() {
            // Only eye moves available and we have 2 or fewer eyes, pass
            return None;
        }

        if valid_moves.is_empty() {
            return None;
        }

        // Run simulations for each valid move
        let mut move_wins = vec![0; valid_moves.len()];
        let mut move_games = vec![0; valid_moves.len()];
        let mut _total_simulations = 0;

        let start_time = Instant::now();

        // Run simulations until time limit
        while start_time.elapsed() < self.time_limit {
            for (idx, &(x, y)) in valid_moves.iter().enumerate() {
                if start_time.elapsed() >= self.time_limit {
                    break;
                }

                // Run one simulation for this move
                let result = self.simulate_game(board, stone, (x, y));

                move_games[idx] += 1;
                if result > 0.5 {
                    move_wins[idx] += 1;
                }
                _total_simulations += 1;
            }
        }

        // Select move with best win rate
        let mut best_idx = 0;
        let mut best_win_rate = 0.0;

        for idx in 0..valid_moves.len() {
            if move_games[idx] > 0 {
                let win_rate = move_wins[idx] as f64 / move_games[idx] as f64;
                if win_rate > best_win_rate {
                    best_win_rate = win_rate;
                    best_idx = idx;
                }
            }
        }

        // Debug output (commented out for performance)
        // println!(
        //     "Monte Carlo: {} simulations, best move win rate: {:.1}% ({}/{})",
        //     total_simulations,
        //     best_win_rate * 100.0,
        //     move_wins[best_idx],
        //     move_games[best_idx]
        // );

        Some(valid_moves[best_idx])
    }
}
