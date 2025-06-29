// Benchmark with more detailed measurements
use jungo::ai::RandomAI;
use jungo::board::{Board, Stone};
use jungo::player::Player;
use std::time::Instant;

fn detailed_playout_benchmark(board_size: usize) {
    println!(
        "\n=== Detailed Playout Analysis for {}x{} ===",
        board_size, board_size
    );

    let mut board = Board::new(board_size);
    let random1 = RandomAI::new();
    let random2 = RandomAI::new();

    let mut current_turn = Stone::Black;
    let mut consecutive_passes = 0;

    let _is_valid_time = 0.0;
    let mut place_stone_time = 0.0;
    let mut get_move_time = 0.0;
    let mut move_count = 0;

    let game_start = Instant::now();

    loop {
        let current_player: &dyn Player = match current_turn {
            Stone::Black => &random1,
            Stone::White => &random2,
        };

        let start = Instant::now();
        let chosen_move = current_player.get_move(&board, current_turn);
        get_move_time += start.elapsed().as_secs_f64();

        match chosen_move {
            Some((x, y)) => {
                let start = Instant::now();
                if board.place_stone(x, y, current_turn).is_ok() {
                    consecutive_passes = 0;
                    move_count += 1;
                }
                place_stone_time += start.elapsed().as_secs_f64();
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

    let total_time = game_start.elapsed().as_secs_f64();

    println!("Total game time: {:.4}s", total_time);
    println!("Total moves: {}", move_count);
    println!(
        "get_move() time: {:.4}s ({:.1}%)",
        get_move_time,
        get_move_time / total_time * 100.0
    );
    println!(
        "place_stone() time: {:.4}s ({:.1}%)",
        place_stone_time,
        place_stone_time / total_time * 100.0
    );
    println!(
        "Other time: {:.4}s ({:.1}%)",
        total_time - get_move_time - place_stone_time,
        (total_time - get_move_time - place_stone_time) / total_time * 100.0
    );
}

fn profile_is_valid_move() {
    println!("\n=== is_valid_move Component Profiling ===");

    let board_size = 19;
    let mut board = Board::new(board_size);

    // Create a more complex board state
    for i in 0..50 {
        let x = (i * 7) % board_size;
        let y = (i * 11) % board_size;
        let stone = if i % 2 == 0 {
            Stone::Black
        } else {
            Stone::White
        };
        let _ = board.place_stone(x, y, stone);
    }

    let iterations = 10000;
    let positions: Vec<(usize, usize)> = (0..100)
        .map(|i| ((i * 3) % board_size, (i * 5) % board_size))
        .collect();

    // Measure different scenarios
    let start = Instant::now();
    let mut valid_count = 0;
    for _ in 0..iterations {
        for &(x, y) in &positions {
            if board.is_valid_move(x, y, Stone::Black) {
                valid_count += 1;
            }
        }
    }
    let total_time = start.elapsed().as_secs_f64();

    println!("Total checks: {}", iterations * positions.len());
    println!("Valid moves found: {}", valid_count);
    println!("Time: {:.4}s", total_time);
    println!(
        "Checks per second: {:.0}",
        (iterations * positions.len()) as f64 / total_time
    );
}

fn main() {
    println!("=== Advanced Performance Analysis ===");

    // Detailed playout analysis
    for &size in &[9, 13, 19] {
        detailed_playout_benchmark(size);
    }

    // Component profiling
    profile_is_valid_move();
}
