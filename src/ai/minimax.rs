use crate::board::{Board, Stone};
use crate::player::Player;

pub struct MinimaxAI {
    name: String,
    max_depth: usize,
    eval_count: std::cell::RefCell<usize>,
}

impl MinimaxAI {
    pub fn new(max_depth: usize) -> Self {
        MinimaxAI {
            name: format!("Minimax AI (depth {})", max_depth),
            max_depth,
            eval_count: std::cell::RefCell::new(0),
        }
    }

    fn evaluate_board(&self, board: &Board, stone: Stone) -> i32 {
        *self.eval_count.borrow_mut() += 1;

        let (black_stones, white_stones) = board.count_stones();
        let (black_captured, white_captured) = board.get_captured();

        // Pure Go scoring: stones on board + captured opponent stones
        let black_score = (black_stones + black_captured) as i32;
        let white_score = (white_stones + white_captured) as i32;

        let material_diff = black_score - white_score;

        // Add eye bonus
        let our_eyes = board.count_eyes_for_color(stone) as i32;
        let their_eyes = board.count_eyes_for_color(stone.opposite()) as i32;
        let eye_bonus = (our_eyes.min(2) - their_eyes.min(2)) * 20;

        // Return score from perspective of current player
        let base_score = match stone {
            Stone::Black => material_diff,
            Stone::White => -material_diff,
        };

        base_score * 100 + eye_bonus
    }

    #[allow(dead_code)]
    fn evaluate_immediate_captures(&self, _board: &Board, _stone: Stone) -> i32 {
        0
    }

    #[allow(dead_code)]
    fn evaluate_position(&self, board: &Board, stone: Stone) -> i32 {
        let mut score = 0;
        let size = board.size();
        let center = size / 2;

        for y in 0..size {
            for x in 0..size {
                if board.get(x, y) == Some(stone) {
                    // Prefer center positions early game
                    let dist_from_center =
                        (x as i32 - center as i32).abs() + (y as i32 - center as i32).abs();
                    score += 5 - dist_from_center.min(5);

                    // Bonus for connected stones
                    let mut connections = 0;
                    if x > 0 && board.get(x - 1, y) == Some(stone) {
                        connections += 1;
                    }
                    if x < size - 1 && board.get(x + 1, y) == Some(stone) {
                        connections += 1;
                    }
                    if y > 0 && board.get(x, y - 1) == Some(stone) {
                        connections += 1;
                    }
                    if y < size - 1 && board.get(x, y + 1) == Some(stone) {
                        connections += 1;
                    }

                    score += connections * 3;
                }
            }
        }

        score
    }

    #[allow(dead_code)]
    fn evaluate_eyes(&self, board: &Board, stone: Stone) -> i32 {
        let our_eyes = board.count_eyes_for_color(stone) as i32;
        let opponent_eyes = board.count_eyes_for_color(stone.opposite()) as i32;

        // Having 2+ eyes is extremely valuable
        let our_eye_score = match our_eyes {
            0 => 0,
            1 => 10,
            2 => 50,                      // Two eyes = life
            _ => 50 + (our_eyes - 2) * 5, // Extra eyes are less valuable
        };

        let opponent_eye_score = match opponent_eyes {
            0 => 0,
            1 => 10,
            2 => 50,
            _ => 50 + (opponent_eyes - 2) * 5,
        };

        our_eye_score - opponent_eye_score
    }

    #[allow(dead_code)]
    fn evaluate_territory(&self, board: &Board, stone: Stone) -> i32 {
        let mut territory_score = 0;
        let opponent = stone.opposite();

        // Count empty spaces near our stones vs opponent stones
        for y in 0..board.size() {
            for x in 0..board.size() {
                if board.get(x, y).is_none() {
                    let mut our_influence = 0;
                    let mut opp_influence = 0;

                    // Check neighbors
                    for (nx, ny) in self.get_neighbors(board, x, y) {
                        match board.get(nx, ny) {
                            Some(s) if s == stone => our_influence += 1,
                            Some(s) if s == opponent => opp_influence += 1,
                            _ => {}
                        }
                    }

                    // Empty space controlled by us
                    if our_influence > opp_influence {
                        territory_score += our_influence;
                    } else if opp_influence > our_influence {
                        territory_score -= opp_influence;
                    }
                }
            }
        }

        territory_score
    }

    #[allow(dead_code)]
    fn evaluate_capture_situations(&self, _board: &Board, _stone: Stone) -> i32 {
        0 // Simplified for performance
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
                let score = if self.max_depth == 1 {
                    self.evaluate_board(&test_board, stone)
                } else {
                    self.minimax(
                        &mut test_board,
                        self.max_depth - 1,
                        false,
                        i32::MIN,
                        i32::MAX,
                        stone.opposite(),
                        stone,
                    )
                };

                if score > best_score {
                    best_score = score;
                    best_move = Some((x, y));
                }
            }
        }

        best_move
    }
}
