use jungo::board::{Board, Stone};
use jungo::fast_board::FastBoard;
use std::time::Instant;

fn benchmark_board_operations(iterations: usize) {
    println!(
        "\n=== Board Operations Benchmark ({}x{}, {} iterations) ===",
        19, 19, iterations
    );

    // Test pattern of moves
    let moves = vec![
        (3, 3),
        (15, 15),
        (3, 15),
        (15, 3),
        (9, 9),
        (4, 3),
        (14, 15),
        (3, 14),
        (15, 4),
        (10, 9),
        (3, 4),
        (15, 14),
        (4, 15),
        (14, 3),
        (9, 10),
    ];

    // Benchmark original Board
    let start = Instant::now();
    let mut original_valid_count = 0;

    for _ in 0..iterations {
        let board = Board::new(19);
        for y in 0..19 {
            for x in 0..19 {
                if board.is_valid_move(x, y, Stone::Black) {
                    original_valid_count += 1;
                }
            }
        }
    }

    let original_time = start.elapsed().as_secs_f64();

    // Benchmark FastBoard
    let start = Instant::now();
    let mut fast_valid_count = 0;

    for _ in 0..iterations {
        let board = FastBoard::new(19);
        for y in 0..19 {
            for x in 0..19 {
                if board.is_valid_move(x, y, Stone::Black) {
                    fast_valid_count += 1;
                }
            }
        }
    }

    let fast_time = start.elapsed().as_secs_f64();

    println!("Original Board:");
    println!("  Time: {:.4}s", original_time);
    println!("  Valid moves: {}", original_valid_count);
    println!(
        "  Checks/second: {:.0}",
        (iterations * 19 * 19) as f64 / original_time
    );

    println!("FastBoard:");
    println!("  Time: {:.4}s", fast_time);
    println!("  Valid moves: {}", fast_valid_count);
    println!(
        "  Checks/second: {:.0}",
        (iterations * 19 * 19) as f64 / fast_time
    );

    println!("Speedup: {:.2}x", original_time / fast_time);

    // Benchmark place_stone operations
    println!("\n=== place_stone Benchmark ===");

    // Original Board
    let start = Instant::now();
    for _ in 0..iterations {
        let mut board = Board::new(19);
        let mut stone = Stone::Black;
        for &(x, y) in &moves {
            let _ = board.place_stone(x, y, stone);
            stone = stone.opposite();
        }
    }
    let original_place_time = start.elapsed().as_secs_f64();

    // FastBoard
    let start = Instant::now();
    for _ in 0..iterations {
        let mut board = FastBoard::new(19);
        let mut stone = Stone::Black;
        for &(x, y) in &moves {
            let _ = board.place_stone(x, y, stone);
            stone = stone.opposite();
        }
    }
    let fast_place_time = start.elapsed().as_secs_f64();

    println!("Original Board place_stone: {:.4}s", original_place_time);
    println!("FastBoard place_stone: {:.4}s", fast_place_time);
    println!("Speedup: {:.2}x", original_place_time / fast_place_time);
}

fn main() {
    println!("=== Board Implementation Comparison ===");

    benchmark_board_operations(100);
}
