use crate::ai::RandomAI;
use crate::board::{Board, Stone};
use crate::player::Player;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct MctsNode {
    visits: u32,
    wins: f64,
    move_pos: Option<(usize, usize)>,
    stone: Stone,
    children: Vec<Rc<RefCell<MctsNode>>>,
    untried_moves: Vec<(usize, usize)>,
    board_before_move: Option<Board>,
}

impl MctsNode {
    fn new(
        stone: Stone,
        move_pos: Option<(usize, usize)>,
        available_moves: Vec<(usize, usize)>,
        board_before_move: Option<Board>,
    ) -> Self {
        MctsNode {
            visits: 0,
            wins: 0.0,
            move_pos,
            stone,
            children: Vec::new(),
            untried_moves: available_moves,
            board_before_move,
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

    fn expand(&mut self, board: &Board, parent_stone: Stone) -> Option<Rc<RefCell<MctsNode>>> {
        if self.untried_moves.is_empty() {
            return None;
        }

        // Pick a random untried move
        let idx = rand::random::<usize>() % self.untried_moves.len();
        let chosen_move = self.untried_moves.remove(idx);

        // Get valid moves for the child node considering Ko rule
        let mut child_board = board.clone();
        if child_board
            .place_stone(chosen_move.0, chosen_move.1, parent_stone)
            .is_ok()
        {
            let child_stone = parent_stone.opposite();
            // Use the board before this move as the previous board for Ko rule checking
            let child_moves = if let Some(ref board_before) = self.board_before_move {
                get_valid_moves_with_ko(&child_board, child_stone, Some(board_before))
            } else {
                get_valid_moves_with_ko(&child_board, child_stone, Some(board))
            };

            let child_node = Rc::new(RefCell::new(MctsNode::new(
                child_stone,
                Some(chosen_move),
                child_moves,
                Some(board.clone()),
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
        let mut previous_board: Option<Board> = Some(board.clone());

        let random1 = RandomAI::new();
        let random2 = RandomAI::new();

        let mut moves = 0;
        let max_moves = board.size() * board.size() * 3;

        loop {
            let current_player: &dyn Player = match current_turn {
                s if s == stone => &random1,
                _ => &random2,
            };

            match current_player.get_move(&sim_board, current_turn) {
                Some((x, y)) => {
                    // Check Ko rule if we have a previous board
                    let is_valid = if let Some(ref prev_board) = previous_board {
                        sim_board.is_valid_move_with_ko(x, y, current_turn, prev_board)
                    } else {
                        sim_board.is_valid_move(x, y, current_turn)
                    };

                    if is_valid {
                        let board_before_move = sim_board.clone();
                        if sim_board.place_stone(x, y, current_turn).is_ok() {
                            consecutive_passes = 0;
                            previous_board = Some(board_before_move);
                        }
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

        let root = Rc::new(RefCell::new(MctsNode::new(stone, None, valid_moves, None)));
        let start_time = Instant::now();
        let mut iterations = 0;

        while start_time.elapsed() < self.time_limit {
            let mut current_board = board.clone();
            let mut current_node = root.clone();
            let mut path = vec![current_node.clone()];
            let mut current_stone = stone;
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
                    current_board
                        .place_stone(child_move.0, child_move.1, current_stone)
                        .unwrap();
                    board_history.push(board_before_move);
                    current_stone = current_stone.opposite();
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
                .expand(&current_board, current_stone)
            {
                let child_move = new_child.borrow().move_pos.unwrap();
                current_board
                    .place_stone(child_move.0, child_move.1, current_stone)
                    .unwrap();
                current_stone = current_stone.opposite();
                path.push(new_child);
            }

            // Simulation phase - play out random game
            let result = self.simulate(&current_board, current_stone);

            // Backpropagation phase - update all nodes in path
            for (i, node) in path.iter().enumerate() {
                let node_stone = node.borrow().stone;
                // Invert result for opponent's nodes
                let node_result = if (i % 2 == 0) == (node_stone == stone) {
                    result
                } else {
                    1.0 - result
                };
                node.borrow_mut().update(node_result);
            }

            iterations += 1;
        }

        // Select best move based on visit count
        let root_ref = root.borrow();
        let best_child = root_ref
            .children
            .iter()
            .max_by_key(|child| child.borrow().visits)
            .cloned()?;

        let best_move = best_child.borrow().move_pos;

        // Debug output
        println!(
            "MCTS: {} iterations, best move visits: {}/{} (win rate: {:.1}%)",
            iterations,
            best_child.borrow().visits,
            root_ref.visits,
            best_child.borrow().wins / best_child.borrow().visits.max(1) as f64 * 100.0
        );

        best_move
    }
}

// Helper function to get valid moves considering eyes and Ko rule
fn get_valid_moves(board: &Board, stone: Stone) -> Vec<(usize, usize)> {
    get_valid_moves_with_ko(board, stone, None)
}

fn get_valid_moves_with_ko(
    board: &Board,
    stone: Stone,
    previous_board: Option<&Board>,
) -> Vec<(usize, usize)> {
    let mut valid_moves = Vec::new();
    let mut non_eye_moves = Vec::new();

    for y in 0..board.size() {
        for x in 0..board.size() {
            let is_valid = if let Some(prev_board) = previous_board {
                board.is_valid_move_with_ko(x, y, stone, prev_board)
            } else {
                board.is_valid_move(x, y, stone)
            };

            if is_valid {
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
