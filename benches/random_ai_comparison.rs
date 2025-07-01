use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use jungo::ai::{RandomAI, RandomAIV2, RandomAIV3, RandomAIV4};
use jungo::board::{Board, Stone};
use jungo::player::Player;

fn setup_empty_board(size: usize) -> Board {
    Board::new(size)
}

fn setup_midgame_board(size: usize) -> Board {
    let mut board = Board::new(size);
    // Fill ~40% of board with alternating stones
    let fill_ratio = 0.4;
    let num_stones = ((size * size) as f64 * fill_ratio) as usize;

    for i in 0..num_stones {
        let x = (i * 7 + i / 3) % size;
        let y = (i * 11 + i / 5) % size;
        let stone = if i % 2 == 0 {
            Stone::Black
        } else {
            Stone::White
        };
        let _ = board.place_stone(x, y, stone);
    }
    board
}

fn setup_endgame_board(size: usize) -> Board {
    let mut board = Board::new(size);
    // Fill most of the board, leaving only a few moves
    for y in 0..size {
        for x in 0..size {
            // Leave some scattered empty spaces
            if (x + y) % 7 != 0 || (x * y) % 11 == 0 {
                let stone = if (x + y) % 2 == 0 {
                    Stone::Black
                } else {
                    Stone::White
                };
                let _ = board.place_stone(x, y, stone);
            }
        }
    }
    board
}

fn bench_random_ai_variants(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_ai_comparison");

    // Test scenarios
    let scenarios = vec![
        (
            "empty",
            Box::new(setup_empty_board) as Box<dyn Fn(usize) -> Board>,
        ),
        (
            "midgame",
            Box::new(setup_midgame_board) as Box<dyn Fn(usize) -> Board>,
        ),
        (
            "endgame",
            Box::new(setup_endgame_board) as Box<dyn Fn(usize) -> Board>,
        ),
    ];

    let sizes = vec![5, 9, 19];

    for (scenario_name, setup_fn) in &scenarios {
        for &size in &sizes {
            let board = setup_fn(size);

            // Test original RandomAI
            group.bench_with_input(
                BenchmarkId::new("RandomAI", format!("{}x{}_{}", size, size, scenario_name)),
                &board,
                |b, board| {
                    let ai = RandomAI::new();
                    b.iter(|| {
                        black_box(ai.get_move(board, Stone::Black));
                    });
                },
            );

            // Test RandomAIV2 (Early Exit)
            group.bench_with_input(
                BenchmarkId::new("RandomAIV2", format!("{}x{}_{}", size, size, scenario_name)),
                &board,
                |b, board| {
                    let ai = RandomAIV2::new();
                    b.iter(|| {
                        black_box(ai.get_move(board, Stone::Black));
                    });
                },
            );

            // Test RandomAIV3 (Reservoir Sampling)
            group.bench_with_input(
                BenchmarkId::new("RandomAIV3", format!("{}x{}_{}", size, size, scenario_name)),
                &board,
                |b, board| {
                    let ai = RandomAIV3::new();
                    b.iter(|| {
                        black_box(ai.get_move(board, Stone::Black));
                    });
                },
            );

            // Test RandomAIV4 (Pre-filtered Empty Cells)
            group.bench_with_input(
                BenchmarkId::new("RandomAIV4", format!("{}x{}_{}", size, size, scenario_name)),
                &board,
                |b, board| {
                    let ai = RandomAIV4::new();
                    b.iter(|| {
                        black_box(ai.get_move(board, Stone::Black));
                    });
                },
            );
        }
    }

    group.finish();
}

// Focused benchmark for 9x9 board only (faster iteration)
fn bench_9x9_detailed(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_ai_9x9_detailed");

    let boards = vec![
        ("empty", setup_empty_board(9)),
        ("midgame", setup_midgame_board(9)),
        ("endgame", setup_endgame_board(9)),
    ];

    for (scenario, board) in &boards {
        let ai_variants: Vec<(&str, Box<dyn Player>)> = vec![
            ("Original", Box::new(RandomAI::new())),
            ("V2_EarlyExit", Box::new(RandomAIV2::new())),
            ("V3_Reservoir", Box::new(RandomAIV3::new())),
            ("V4_PreFilter", Box::new(RandomAIV4::new())),
        ];

        for (name, ai) in &ai_variants {
            group.bench_with_input(BenchmarkId::new(*name, scenario), board, |b, board| {
                b.iter(|| {
                    black_box(ai.get_move(board, Stone::Black));
                });
            });
        }
    }

    group.finish();
}

criterion_group!(benches, bench_random_ai_variants, bench_9x9_detailed);
criterion_main!(benches);
