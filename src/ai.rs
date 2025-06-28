use crate::board::{Board, Stone};
use crate::player::Player;
use rand::Rng;
use std::time::{Duration, Instant};

pub struct RandomAI {
    name: String,
}

impl RandomAI {
    pub fn new() -> Self {
        RandomAI {
            name: "Random AI".to_string(),
        }
    }
}

impl Default for RandomAI {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for RandomAI {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&self, board: &Board, _stone: Stone) -> Option<(usize, usize)> {
        let mut rng = rand::thread_rng();
        let mut valid_moves = Vec::new();

        // Collect all valid moves
        for y in 0..board.size() {
            for x in 0..board.size() {
                if board.is_valid_move(x, y) {
                    valid_moves.push((x, y));
                }
            }
        }

        if valid_moves.is_empty() {
            return None; // Pass
        }

        // Randomly select a move (80% chance) or pass (20% chance)
        if rng.gen_bool(0.8) {
            let index = rng.gen_range(0..valid_moves.len());
            Some(valid_moves[index])
        } else {
            None
        }
    }
}

pub struct MinimaxAI {
    name: String,
    max_depth: usize,
}

impl MinimaxAI {
    pub fn new(max_depth: usize) -> Self {
        MinimaxAI {
            name: format!("Minimax AI (depth {})", max_depth),
            max_depth,
        }
    }

    fn evaluate_board(&self, board: &Board, stone: Stone) -> i32 {
        let (black_stones, white_stones) = board.count_stones();
        let (black_captured, white_captured) = board.get_captured();

        let black_score = (black_stones + black_captured) as i32;
        let white_score = (white_stones + white_captured) as i32;

        match stone {
            Stone::Black => black_score - white_score,
            Stone::White => white_score - black_score,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn minimax(
        &self,
        board: &mut Board,
        depth: usize,
        is_maximizing: bool,
        alpha: i32,
        beta: i32,
        stone: Stone,
        original_stone: Stone,
    ) -> i32 {
        if depth == 0 {
            return self.evaluate_board(board, original_stone);
        }

        let mut valid_moves = Vec::new();
        for y in 0..board.size() {
            for x in 0..board.size() {
                if board.is_valid_move(x, y) {
                    valid_moves.push((x, y));
                }
            }
        }

        if valid_moves.is_empty() {
            return self.evaluate_board(board, original_stone);
        }

        let mut alpha = alpha;
        let mut beta = beta;

        if is_maximizing {
            let mut max_eval = i32::MIN;
            for (x, y) in valid_moves {
                let mut new_board = board.clone();
                if new_board.place_stone(x, y, stone).is_ok() {
                    let eval = self.minimax(
                        &mut new_board,
                        depth - 1,
                        false,
                        alpha,
                        beta,
                        stone.opposite(),
                        original_stone,
                    );
                    max_eval = max_eval.max(eval);
                    alpha = alpha.max(eval);
                    if beta <= alpha {
                        break; // Beta pruning
                    }
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for (x, y) in valid_moves {
                let mut new_board = board.clone();
                if new_board.place_stone(x, y, stone).is_ok() {
                    let eval = self.minimax(
                        &mut new_board,
                        depth - 1,
                        true,
                        alpha,
                        beta,
                        stone.opposite(),
                        original_stone,
                    );
                    min_eval = min_eval.min(eval);
                    beta = beta.min(eval);
                    if beta <= alpha {
                        break; // Alpha pruning
                    }
                }
            }
            min_eval
        }
    }
}

impl Player for MinimaxAI {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&self, board: &Board, stone: Stone) -> Option<(usize, usize)> {
        let mut valid_moves = Vec::new();
        for y in 0..board.size() {
            for x in 0..board.size() {
                if board.is_valid_move(x, y) {
                    valid_moves.push((x, y));
                }
            }
        }

        if valid_moves.is_empty() {
            return None;
        }

        let mut best_move = None;
        let mut best_score = i32::MIN;

        for (x, y) in valid_moves {
            let mut test_board = board.clone();
            if test_board.place_stone(x, y, stone).is_ok() {
                let score = self.minimax(
                    &mut test_board,
                    self.max_depth - 1,
                    false,
                    i32::MIN,
                    i32::MAX,
                    stone.opposite(),
                    stone,
                );
                if score > best_score {
                    best_score = score;
                    best_move = Some((x, y));
                }
            }
        }

        best_move
    }
}

pub struct Mcts {
    name: String,
    time_limit: Duration,
}

impl Mcts {
    pub fn new(time_seconds: u64) -> Self {
        Mcts {
            name: format!("MCTS AI ({}s)", time_seconds),
            time_limit: Duration::from_secs(time_seconds),
        }
    }

    fn simulate(&self, board: &Board, stone: Stone) -> i32 {
        let mut sim_board = board.clone();
        let mut current_stone = stone;
        let mut rng = rand::thread_rng();
        let mut consecutive_passes = 0;

        loop {
            let mut valid_moves = Vec::new();
            for y in 0..sim_board.size() {
                for x in 0..sim_board.size() {
                    if sim_board.is_valid_move(x, y) {
                        valid_moves.push((x, y));
                    }
                }
            }

            if valid_moves.is_empty() || (consecutive_passes < 2 && rng.gen_bool(0.1)) {
                consecutive_passes += 1;
                if consecutive_passes >= 2 {
                    break;
                }
            } else {
                consecutive_passes = 0;
                let idx = rng.gen_range(0..valid_moves.len());
                let (x, y) = valid_moves[idx];
                let _ = sim_board.place_stone(x, y, current_stone);
            }

            current_stone = current_stone.opposite();
        }

        let (black_stones, white_stones) = sim_board.count_stones();
        let (black_captured, white_captured) = sim_board.get_captured();

        let black_score = (black_stones + black_captured) as i32;
        let white_score = (white_stones + white_captured) as i32;

        match stone {
            Stone::Black => {
                if black_score > white_score {
                    1
                } else {
                    -1
                }
            }
            Stone::White => {
                if white_score > black_score {
                    1
                } else {
                    -1
                }
            }
        }
    }
}

impl Player for Mcts {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&self, board: &Board, stone: Stone) -> Option<(usize, usize)> {
        let mut valid_moves = Vec::new();
        for y in 0..board.size() {
            for x in 0..board.size() {
                if board.is_valid_move(x, y) {
                    valid_moves.push((x, y));
                }
            }
        }

        if valid_moves.is_empty() {
            return None;
        }

        let start_time = Instant::now();
        let mut move_scores = vec![0i32; valid_moves.len()];
        let mut move_counts = vec![0u32; valid_moves.len()];
        let mut iterations = 0;

        while start_time.elapsed() < self.time_limit {
            for (idx, &(x, y)) in valid_moves.iter().enumerate() {
                let mut test_board = board.clone();
                if test_board.place_stone(x, y, stone).is_ok() {
                    let result = self.simulate(&test_board, stone);
                    move_scores[idx] += result;
                    move_counts[idx] += 1;
                }
            }
            iterations += 1;
        }

        println!(
            "MCTS completed {} iterations",
            iterations * valid_moves.len()
        );

        let mut best_idx = 0;
        let mut best_win_rate = f64::MIN;

        for (idx, &count) in move_counts.iter().enumerate() {
            if count > 0 {
                let win_rate = move_scores[idx] as f64 / count as f64;
                if win_rate > best_win_rate {
                    best_win_rate = win_rate;
                    best_idx = idx;
                }
            }
        }

        Some(valid_moves[best_idx])
    }
}
