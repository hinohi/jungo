use crate::ai::LightRandomAI;
use crate::board::{Board, Stone};
use crate::player::Player;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct MctsNode {
    visits: u32,
    wins: f64,
    move_pos: Option<(usize, usize)>, // The move that led to this position (None for root)
    player_to_move: Stone,            // Whose turn it is to play FROM this position
    children: Vec<Rc<RefCell<MctsNode>>>,
    untried_moves: Vec<(usize, usize)>,
}

impl MctsNode {
    fn new(
        player_to_move: Stone,
        move_pos: Option<(usize, usize)>,
        available_moves: Vec<(usize, usize)>,
    ) -> Self {
        MctsNode {
            visits: 0,
            wins: 0.0,
            move_pos,
            player_to_move,
            children: Vec::new(),
            untried_moves: available_moves,
        }
    }

    fn uct_value(&self, parent_visits: u32, exploration: f64) -> f64 {
        if self.visits == 0 {
            f64::INFINITY
        } else {
            let win_rate = self.wins / self.visits as f64;
            let exploration_term =
                exploration * ((parent_visits as f64).ln() / self.visits as f64).sqrt();
            win_rate + exploration_term
        }
    }

    fn select_child(&self, exploration: f64) -> Option<Rc<RefCell<MctsNode>>> {
        if self.children.is_empty() {
            return None;
        }

        self.children
            .iter()
            .max_by(|a, b| {
                let a_val = a.borrow().uct_value(self.visits, exploration);
                let b_val = b.borrow().uct_value(self.visits, exploration);
                a_val.partial_cmp(&b_val).unwrap()
            })
            .cloned()
    }

    fn expand(&mut self, board: &Board, current_player: Stone) -> Option<Rc<RefCell<MctsNode>>> {
        if self.untried_moves.is_empty() {
            return None;
        }

        // Pick a random untried move
        let idx = rand::random::<usize>() % self.untried_moves.len();
        let chosen_move = self.untried_moves.remove(idx);

        // Get valid moves for the child node
        let mut child_board = board.clone();
        // Place stone for the current player (whose turn it is from this node)
        if child_board
            .place_stone(chosen_move.0, chosen_move.1, current_player)
            .is_ok()
        {
            // Child will be opponent's turn
            let child_stone = current_player.opposite();
            // Ko rule is handled at the Game level, not in MCTS
            let child_moves = get_valid_moves(&child_board, child_stone);

            let child_node = Rc::new(RefCell::new(MctsNode::new(
                child_stone,
                Some(chosen_move),
                child_moves,
            )));

            self.children.push(child_node.clone());
            Some(child_node)
        } else {
            None
        }
    }

    fn update(&mut self, result: f64) {
        self.visits += 1;
        self.wins += result;
    }
}

pub struct Mcts {
    name: String,
    time_limit: Duration,
    exploration: f64,
}

impl Mcts {
    pub fn new(time_seconds: u64) -> Self {
        Mcts {
            name: format!("MCTS AI ({}s)", time_seconds),
            time_limit: Duration::from_secs(time_seconds),
            exploration: 1.4, // Standard UCT exploration constant
        }
    }

    fn simulate(&self, board: &Board, stone: Stone) -> f64 {
        let mut sim_board = board.clone();
        let mut current_turn = stone;
        let mut consecutive_passes = 0;

        // Use lightweight random players for faster simulation
        let mut random1 = LightRandomAI::new();
        let mut random2 = LightRandomAI::new();

        let mut moves = 0;
        let max_moves = board.size() * board.size(); // Further reduced

        loop {
            let valid_move = match current_turn {
                s if s == stone => random1.get_fast_move(&sim_board, current_turn),
                _ => random2.get_fast_move(&sim_board, current_turn),
            };

            match valid_move {
                Some((x, y)) => {
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

    fn run_mcts(&self, board: &Board, stone: Stone) -> Option<(usize, usize)> {
        // For the root, we don't have Ko information, so we use basic validation
        let valid_moves = get_valid_moves(board, stone);

        if valid_moves.is_empty() {
            return None;
        }

        if valid_moves.len() == 1 {
            return Some(valid_moves[0]);
        }

        let root = Rc::new(RefCell::new(MctsNode::new(stone, None, valid_moves)));
        let start_time = Instant::now();
        let mut _iterations = 0;

        while start_time.elapsed() < self.time_limit {
            let mut current_board = board.clone();
            let mut current_node = root.clone();
            let mut path = vec![current_node.clone()];
            // Track whose turn it is to play from the current position
            let mut current_player = stone;
            let mut board_history = vec![board.clone()];

            // Selection phase - traverse tree using UCT
            loop {
                let node = current_node.borrow();

                if !node.untried_moves.is_empty() || node.children.is_empty() {
                    drop(node);
                    break;
                }

                if let Some(child) = node.select_child(self.exploration) {
                    let child_move = child.borrow().move_pos.unwrap();
                    let board_before_move = current_board.clone();
                    // Play move for current player
                    current_board
                        .place_stone(child_move.0, child_move.1, current_player)
                        .unwrap();
                    board_history.push(board_before_move);
                    // Now it's opponent's turn
                    current_player = current_player.opposite();
                    drop(node);
                    current_node = child;
                    path.push(current_node.clone());
                } else {
                    drop(node);
                    break;
                }
            }

            // Expansion phase - add new child if possible
            if let Some(new_child) = current_node
                .borrow_mut()
                .expand(&current_board, current_player)
            {
                let child_move = new_child.borrow().move_pos.unwrap();
                current_board
                    .place_stone(child_move.0, child_move.1, current_player)
                    .unwrap();
                // After expansion, it's opponent's turn for simulation
                current_player = current_player.opposite();
                path.push(new_child);
            }

            // Simulation phase - play out random game
            // current_player is whose turn it is to play from current position
            let result = self.simulate(&current_board, current_player);

            // Backpropagation phase - update all nodes in path
            // Result is from the perspective of the player who moves first in simulation
            // We propagate the result up the tree, flipping perspective at each level
            let simulation_winner = if result > 0.5 {
                current_player
            } else {
                current_player.opposite()
            };

            for node in path.iter() {
                let node_player = node.borrow().player_to_move;
                // This node represents a position where node_player is about to move
                // If the simulation winner is node_player, this is a good position for them
                let node_result = if node_player == simulation_winner {
                    1.0
                } else {
                    0.0
                };
                node.borrow_mut().update(node_result);
            }

            _iterations += 1;
        }

        // Select best move based on visit count
        let root_ref = root.borrow();
        let best_child = root_ref
            .children
            .iter()
            .max_by_key(|child| child.borrow().visits)
            .cloned()?;

        let best_move = best_child.borrow().move_pos;

        // Debug output (commented for performance)
        // let visits = best_child.borrow().visits;
        // let wins = best_child.borrow().wins;
        // println!(
        //     "MCTS: {} iterations, best move visits: {}/{} (wins: {:.1}/{}, win rate: {:.1}%)",
        //     _iterations,
        //     visits,
        //     root_ref.visits,
        //     wins,
        //     visits,
        //     wins / visits.max(1) as f64 * 100.0
        // );

        best_move
    }
}

// Helper function to get valid moves considering eyes
fn get_valid_moves(board: &Board, stone: Stone) -> Vec<(usize, usize)> {
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
        non_eye_moves
    } else {
        valid_moves
    }
}

impl Player for Mcts {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&self, board: &Board, stone: Stone) -> Option<(usize, usize)> {
        self.run_mcts(board, stone)
    }
}
