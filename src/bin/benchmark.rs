use jungo::ai::RandomAI;
use jungo::board::{Board, Stone};
use jungo::player::Player;
use rand::{rngs::StdRng, SeedableRng};
use std::time::Instant;

// Fixed seed RandomAI for reproducible benchmarks
#[allow(dead_code)]
struct SeededRandomAI {
    rng: StdRng,
}

#[allow(dead_code)]
impl SeededRandomAI {
    fn new(seed: u64) -> Self {
        SeededRandomAI {
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl Player for SeededRandomAI {
    fn name(&self) -> &str {
        "Seeded Random AI"
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

        let total_eyes = board.count_eyes_for_color(stone);
        let moves = if total_eyes <= 2 && !non_eye_moves.is_empty() {
            non_eye_moves
        } else if total_eyes <= 2 && non_eye_moves.is_empty() {
            return None;
        } else {
            valid_moves
        };

        if moves.is_empty() {
            None
        } else {
            // Use thread_rng for now since we can't mutate self
            let idx = rand::random::<usize>() % moves.len();
            Some(moves[idx])
        }
    }
}

fn benchmark_single_playout(board_size: usize) -> (f64, usize) {
    let mut board = Board::new(board_size);
    let random1 = RandomAI::new();
    let random2 = RandomAI::new();

    let mut current_turn = Stone::Black;
    let mut consecutive_passes = 0;
    let mut move_count = 0;

    let start = Instant::now();

    loop {
        let current_player: &dyn Player = match current_turn {
            Stone::Black => &random1,
            Stone::White => &random2,
        };

        match current_player.get_move(&board, current_turn) {
            Some((x, y)) => {
                if board.place_stone(x, y, current_turn).is_ok() {
                    consecutive_passes = 0;
                    move_count += 1;
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
    }

    let elapsed = start.elapsed().as_secs_f64();
    (elapsed, move_count)
}

fn benchmark_is_valid_move(board_size: usize, iterations: usize) -> f64 {
    let mut board = Board::new(board_size);

    // Place some stones to create a more realistic board state
    let positions = [
        (3, 3),
        (3, 4),
        (4, 3),
        (4, 4),
        (10, 10),
        (10, 11),
        (11, 10),
        (11, 11),
        (16, 16),
        (16, 17),
        (17, 16),
        (17, 17),
    ];

    for (i, &(x, y)) in positions.iter().enumerate() {
        if x < board_size && y < board_size {
            let stone = if i % 2 == 0 {
                Stone::Black
            } else {
                Stone::White
            };
            let _ = board.place_stone(x, y, stone);
        }
    }

    let start = Instant::now();
    let mut valid_count = 0;

    for _ in 0..iterations {
        for y in 0..board_size {
            for x in 0..board_size {
                if board.is_valid_move(x, y, Stone::Black) {
                    valid_count += 1;
                }
            }
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    println!("  Valid moves found: {}", valid_count);
    elapsed
}

fn benchmark_place_stone(board_size: usize) -> f64 {
    let start = Instant::now();

    // Simulate a game with predetermined moves
    let mut board = Board::new(board_size);
    let mut moves_made = 0;

    // Use a simple pattern to place stones
    for i in 0..100 {
        let x = (i * 7) % board_size;
        let y = (i * 11) % board_size;
        let stone = if i % 2 == 0 {
            Stone::Black
        } else {
            Stone::White
        };

        if board.is_valid_move(x, y, stone) && board.place_stone(x, y, stone).is_ok() {
            moves_made += 1;
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    println!("  Moves made: {}", moves_made);
    elapsed
}

fn main() {
    println!("=== Jungo Performance Benchmark ===\n");

    let board_sizes = vec![9, 13, 19];

    // Benchmark 1: Full game playouts
    println!("Benchmark 1: Full Random Game Playouts");
    println!("Board Size | Time (s) | Moves | Moves/sec");
    println!("-----------|----------|-------|----------");

    for &size in &board_sizes {
        let mut total_time = 0.0;
        let mut total_moves = 0;
        let iterations = 10;

        for _ in 0..iterations {
            let (time, moves) = benchmark_single_playout(size);
            total_time += time;
            total_moves += moves;
        }

        let avg_time = total_time / iterations as f64;
        let avg_moves = total_moves as f64 / iterations as f64;
        let moves_per_sec = avg_moves / avg_time;

        println!(
            "{:10} | {:8.4} | {:5.0} | {:9.0}",
            format!("{}x{}", size, size),
            avg_time,
            avg_moves,
            moves_per_sec
        );
    }

    // Benchmark 2: is_valid_move performance
    println!("\nBenchmark 2: is_valid_move Performance");
    println!("Board Size | Iterations | Total Checks | Time (s) | Checks/sec");
    println!("-----------|------------|--------------|----------|------------");

    for &size in &board_sizes {
        let iterations = if size <= 9 {
            100
        } else if size <= 13 {
            50
        } else {
            20
        };
        let total_checks = iterations * size * size;
        let time = benchmark_is_valid_move(size, iterations);
        let checks_per_sec = total_checks as f64 / time;

        println!(
            "{:10} | {:10} | {:12} | {:8.4} | {:10.0}",
            format!("{}x{}", size, size),
            iterations,
            total_checks,
            time,
            checks_per_sec
        );
    }

    // Benchmark 3: place_stone performance
    println!("\nBenchmark 3: place_stone Performance");
    println!("Board Size | Time (s)");
    println!("-----------|----------");

    for &size in &board_sizes {
        let time = benchmark_place_stone(size);
        println!("{:10} | {:8.4}", format!("{}x{}", size, size), time);
    }

    // Benchmark 4: Critical operations breakdown
    println!("\nBenchmark 4: Critical Operations (19x19 board, 1000 iterations)");
    let board_size = 19;
    let iterations = 1000;

    // Test empty board checks
    let board = Board::new(board_size);
    let start = Instant::now();
    for _ in 0..iterations {
        for y in 0..board_size {
            for x in 0..board_size {
                let _ = board.get(x, y);
            }
        }
    }
    let get_time = start.elapsed().as_secs_f64();
    println!(
        "  get() calls: {:.4}s ({:.0} calls/sec)",
        get_time,
        (iterations * board_size * board_size) as f64 / get_time
    );

    // Test is_eye performance
    let start = Instant::now();
    for _ in 0..iterations {
        for y in 0..board_size {
            for x in 0..board_size {
                let _ = board.is_eye(x, y, Stone::Black);
            }
        }
    }
    let eye_time = start.elapsed().as_secs_f64();
    println!(
        "  is_eye() calls: {:.4}s ({:.0} calls/sec)",
        eye_time,
        (iterations * board_size * board_size) as f64 / eye_time
    );
}

// Additional benchmarks we could add:
// 1. Ko rule checking performance
// 2. Eye detection performance
// 3. Group liberty counting performance
// 4. Memory usage profiling
// 5. MCTS tree operations performance
// 6. Parallel playout performance
