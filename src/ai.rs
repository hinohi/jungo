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
        let mut valid_moves = Vec::new();
        let mut non_eye_moves = Vec::new();
        let mut eye_moves = Vec::new();

        // Collect all valid moves and categorize them
        for y in 0..board.size() {
            for x in 0..board.size() {
                if board.is_valid_move(x, y, _stone) {
                    valid_moves.push((x, y));

                    if board.is_eye(x, y, _stone) {
                        eye_moves.push((x, y));
                    } else {
                        non_eye_moves.push((x, y));
                    }
                }
            }
        }

        // If there are no valid moves at all, pass
        if valid_moves.is_empty() {
            return None;
        }

        // Count total eyes for our color
        let total_eyes = board.count_eyes_for_color(_stone);

        // If we have more than 2 eyes, we can fill some
        if total_eyes > 2 && !eye_moves.is_empty() {
            // Prefer non-eye moves, but also consider filling excess eyes
            if !non_eye_moves.is_empty() {
                // 80% chance to play non-eye move, 20% to fill an eye
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.8) {
                    let index = rng.gen_range(0..non_eye_moves.len());
                    return Some(non_eye_moves[index]);
                } else {
                    let index = rng.gen_range(0..eye_moves.len());
                    return Some(eye_moves[index]);
                }
            } else {
                // Only eye moves available, and we have more than 2 eyes, so fill one
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..eye_moves.len());
                return Some(eye_moves[index]);
            }
        }

        // If we have 2 or fewer eyes, don't fill them
        if non_eye_moves.is_empty() {
            return None; // Pass to preserve our eyes
        }

        // Play a non-eye move
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..non_eye_moves.len());
        Some(non_eye_moves[index])
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

        // Basic score from stones and captures
        let black_score = (black_stones + black_captured) as i32;
        let white_score = (white_stones + white_captured) as i32;
        let basic_score = match stone {
            Stone::Black => black_score - white_score,
            Stone::White => white_score - black_score,
        };

        // Additional strategic factors
        let mut bonus = 0;

        // 1. Check for groups in atari (can be captured next move)
        let atari_bonus = self.evaluate_atari_situations(board, stone);
        bonus += atari_bonus;

        // 2. Group safety (number of liberties)
        let safety_bonus = self.evaluate_group_safety(board, stone);
        bonus += safety_bonus;

        // 3. Potential captures
        let capture_potential = self.evaluate_capture_potential(board, stone);
        bonus += capture_potential;

        basic_score * 10 + bonus
    }

    fn evaluate_atari_situations(&self, board: &Board, stone: Stone) -> i32 {
        let mut atari_score = 0;
        let opponent = stone.opposite();

        // Check all opponent groups for atari (1 liberty)
        for y in 0..board.size() {
            for x in 0..board.size() {
                if board.get(x, y) == Some(opponent) {
                    let group = self.get_group(board, x, y);
                    let liberties = self.count_liberties(board, &group);

                    if liberties == 1 {
                        // Opponent group in atari - we can capture it
                        atari_score += group.len() as i32 * 5;
                    }
                }
            }
        }

        // Check our own groups for atari (negative score)
        for y in 0..board.size() {
            for x in 0..board.size() {
                if board.get(x, y) == Some(stone) {
                    let group = self.get_group(board, x, y);
                    let liberties = self.count_liberties(board, &group);

                    if liberties == 1 {
                        // Our group in atari - it can be captured
                        atari_score -= group.len() as i32 * 5;
                    }
                }
            }
        }

        atari_score
    }

    fn evaluate_group_safety(&self, board: &Board, stone: Stone) -> i32 {
        let mut safety_score = 0;

        // Evaluate safety of our groups
        let mut visited = vec![vec![false; board.size()]; board.size()];

        for y in 0..board.size() {
            for x in 0..board.size() {
                if !visited[y][x] && board.get(x, y) == Some(stone) {
                    let group = self.get_group(board, x, y);
                    let liberties = self.count_liberties(board, &group);

                    // Mark group as visited
                    for &(gx, gy) in &group {
                        visited[gy][gx] = true;
                    }

                    // More liberties = safer group
                    safety_score += match liberties {
                        0 => -100, // Should not happen
                        1 => -20,  // In atari
                        2 => 1,    // Vulnerable
                        3 => 3,    // Relatively safe
                        _ => 5,    // Very safe
                    };
                }
            }
        }

        safety_score
    }

    fn evaluate_capture_potential(&self, board: &Board, stone: Stone) -> i32 {
        let mut potential_score = 0;
        let opponent = stone.opposite();

        // Look for opponent groups with few liberties
        let mut visited = vec![vec![false; board.size()]; board.size()];

        for y in 0..board.size() {
            for x in 0..board.size() {
                if !visited[y][x] && board.get(x, y) == Some(opponent) {
                    let group = self.get_group(board, x, y);
                    let liberties = self.count_liberties(board, &group);

                    // Mark group as visited
                    for &(gx, gy) in &group {
                        visited[gy][gx] = true;
                    }

                    // Groups with fewer liberties are easier to capture
                    if liberties == 2 {
                        potential_score += group.len() as i32 * 2;
                    } else if liberties == 3 {
                        potential_score += group.len() as i32;
                    }
                }
            }
        }

        potential_score
    }

    fn get_group(&self, board: &Board, x: usize, y: usize) -> Vec<(usize, usize)> {
        let stone = match board.get(x, y) {
            Some(s) => s,
            None => return vec![],
        };

        let mut group = Vec::new();
        let mut visited = vec![vec![false; board.size()]; board.size()];
        let mut stack = vec![(x, y)];

        while let Some((cx, cy)) = stack.pop() {
            if visited[cy][cx] {
                continue;
            }

            visited[cy][cx] = true;
            group.push((cx, cy));

            let neighbors = self.get_neighbors(board, cx, cy);
            for (nx, ny) in neighbors {
                if !visited[ny][nx] && board.get(nx, ny) == Some(stone) {
                    stack.push((nx, ny));
                }
            }
        }

        group
    }

    fn count_liberties(&self, board: &Board, group: &[(usize, usize)]) -> usize {
        let mut liberties = std::collections::HashSet::new();

        for &(x, y) in group {
            let neighbors = self.get_neighbors(board, x, y);
            for (nx, ny) in neighbors {
                if board.get(nx, ny).is_none() {
                    liberties.insert((nx, ny));
                }
            }
        }

        liberties.len()
    }

    fn get_neighbors(&self, board: &Board, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < board.size() - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < board.size() - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
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
                if board.is_valid_move(x, y, stone) {
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
                if board.is_valid_move(x, y, stone) {
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
                    if sim_board.is_valid_move(x, y, current_stone) {
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
                if board.is_valid_move(x, y, stone) {
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
