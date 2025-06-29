use jungo::ai::{FastRandomAI, RandomAI};
use jungo::board::{Board, Stone};
use jungo::player::Player;
use std::time::Instant;

fn benchmark_ai_comparison(board_size: usize) {
    println!(
        "\n=== AI Performance Comparison for {}x{} ===",
        board_size, board_size
    );

    // Benchmark RandomAI
    let mut total_time_random = 0.0;
    let mut total_moves_random = 0;
    let iterations = 1; // Reduced to 1 iteration for all sizes

    println!("  Benchmarking RandomAI...");
    for _ in 0..iterations {
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

        total_time_random += start.elapsed().as_secs_f64();
        total_moves_random += move_count;
    }

    // Benchmark FastRandomAI
    let mut total_time_fast = 0.0;
    let mut total_moves_fast = 0;

    println!("  Benchmarking FastRandomAI...");
    for _ in 0..iterations {
        let mut board = Board::new(board_size);
        let fast1 = FastRandomAI::new();
        let fast2 = FastRandomAI::new();

        let mut current_turn = Stone::Black;
        let mut consecutive_passes = 0;
        let mut move_count = 0;

        let start = Instant::now();

        loop {
            let current_player: &dyn Player = match current_turn {
                Stone::Black => &fast1,
                Stone::White => &fast2,
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

        total_time_fast += start.elapsed().as_secs_f64();
        total_moves_fast += move_count;
    }

    let avg_time_random = total_time_random / iterations as f64;
    let avg_moves_random = total_moves_random as f64 / iterations as f64;
    let avg_time_fast = total_time_fast / iterations as f64;
    let avg_moves_fast = total_moves_fast as f64 / iterations as f64;

    println!("RandomAI:");
    println!("  Average time: {:.4}s", avg_time_random);
    println!("  Average moves: {:.0}", avg_moves_random);
    println!("  Moves/second: {:.0}", avg_moves_random / avg_time_random);

    println!("FastRandomAI:");
    println!("  Average time: {:.4}s", avg_time_fast);
    println!("  Average moves: {:.0}", avg_moves_fast);
    println!("  Moves/second: {:.0}", avg_moves_fast / avg_time_fast);

    println!("Speedup: {:.2}x", avg_time_random / avg_time_fast);
}

fn main() {
    println!("=== Random AI Optimization Benchmark ===");

    // Only test 9x9 first to see performance difference
    benchmark_ai_comparison(9);
}
