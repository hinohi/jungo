use jungo::ai::{FastRandomAI, RandomAI};
use jungo::board::{Board, Stone};
use jungo::player::Player;
use std::time::Instant;

fn benchmark_move_selection(board_size: usize, iterations: usize) {
    println!(
        "\n=== Move Selection Performance for {}x{} ===",
        board_size, board_size
    );

    // Set up a partially filled board for more realistic testing
    let mut board = Board::new(board_size);

    // Add some stones to make it more realistic
    let positions = [
        (board_size / 4, board_size / 4, Stone::Black),
        (board_size * 3 / 4, board_size / 4, Stone::White),
        (board_size / 4, board_size * 3 / 4, Stone::White),
        (board_size * 3 / 4, board_size * 3 / 4, Stone::Black),
        (board_size / 2, board_size / 2, Stone::Black),
    ];

    for &(x, y, stone) in &positions {
        if x < board_size && y < board_size {
            let _ = board.place_stone(x, y, stone);
        }
    }

    // Benchmark RandomAI
    let random_ai = RandomAI::new();
    let start = Instant::now();
    let mut random_moves = 0;

    for i in 0..iterations {
        let stone = if i % 2 == 0 {
            Stone::Black
        } else {
            Stone::White
        };
        if random_ai.get_move(&board, stone).is_some() {
            random_moves += 1;
        }
    }

    let random_time = start.elapsed().as_secs_f64();

    // Benchmark FastRandomAI
    let fast_ai = FastRandomAI::new();
    let start = Instant::now();
    let mut fast_moves = 0;

    for i in 0..iterations {
        let stone = if i % 2 == 0 {
            Stone::Black
        } else {
            Stone::White
        };
        if fast_ai.get_move(&board, stone).is_some() {
            fast_moves += 1;
        }
    }

    let fast_time = start.elapsed().as_secs_f64();

    println!("RandomAI:");
    println!("  Time: {:.4}s", random_time);
    println!("  Moves found: {}/{}", random_moves, iterations);
    println!("  Moves/second: {:.0}", iterations as f64 / random_time);

    println!("FastRandomAI:");
    println!("  Time: {:.4}s", fast_time);
    println!("  Moves found: {}/{}", fast_moves, iterations);
    println!("  Moves/second: {:.0}", iterations as f64 / fast_time);

    println!("Speedup: {:.2}x", random_time / fast_time);
}

fn main() {
    println!("=== Move Selection Benchmark ===");

    // Test different board sizes with varying iterations
    benchmark_move_selection(9, 10000);
    benchmark_move_selection(13, 5000);
    benchmark_move_selection(19, 2000);
}
