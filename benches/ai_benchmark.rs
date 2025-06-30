use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use jungo::ai::{LightRandomAI, RandomAI};
use jungo::board::{Board, Stone};
use jungo::player::Player;

fn bench_ai_move_selection(c: &mut Criterion) {
    let mut group = c.benchmark_group("ai_move_selection");

    // Create boards with different fill levels
    let board_sizes = vec![9, 13, 19];

    for size in board_sizes {
        // Setup a partially filled board
        let mut board = Board::new(size);

        // Add some stones to make it more realistic
        let num_stones = (size * size) / 4;
        for i in 0..num_stones {
            let x = (i * 7) % size;
            let y = (i * 11) % size;
            let stone = if i % 2 == 0 {
                Stone::Black
            } else {
                Stone::White
            };
            let _ = board.place_stone(x, y, stone);
        }

        group.bench_with_input(
            BenchmarkId::new("RandomAI", format!("{}x{}", size, size)),
            &board,
            |b, board| {
                let ai = RandomAI::new();
                b.iter(|| {
                    black_box(ai.get_move(board, Stone::Black));
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("LightRandomAI", format!("{}x{}", size, size)),
            &board,
            |b, board| {
                let ai = LightRandomAI::new();
                b.iter(|| {
                    black_box(ai.get_move(board, Stone::Black));
                });
            },
        );
    }

    group.finish();
}

fn bench_ai_empty_board(c: &mut Criterion) {
    let mut group = c.benchmark_group("ai_empty_board");

    let board_sizes = vec![9, 13, 19];

    for size in board_sizes {
        let board = Board::new(size);

        group.bench_with_input(
            BenchmarkId::new("RandomAI", format!("{}x{}", size, size)),
            &board,
            |b, board| {
                let ai = RandomAI::new();
                b.iter(|| {
                    black_box(ai.get_move(board, Stone::Black));
                });
            },
        );
    }

    group.finish();
}

fn bench_ai_dense_board(c: &mut Criterion) {
    let mut group = c.benchmark_group("ai_dense_board");

    // Create a very dense board (80% filled)
    let size = 19;
    let mut board = Board::new(size);

    let positions: Vec<(usize, usize)> = (0..size)
        .flat_map(|y| (0..size).map(move |x| (x, y)))
        .collect();

    // Fill 80% of the board
    let fill_count = (positions.len() * 4) / 5;
    for (i, &(x, y)) in positions.iter().enumerate().take(fill_count) {
        let stone = if i % 2 == 0 {
            Stone::Black
        } else {
            Stone::White
        };
        let _ = board.place_stone(x, y, stone);
    }

    group.bench_function("RandomAI_19x19_dense", |b| {
        let ai = RandomAI::new();
        b.iter(|| {
            black_box(ai.get_move(&board, Stone::Black));
        });
    });

    group.bench_function("LightRandomAI_19x19_dense", |b| {
        let ai = LightRandomAI::new();
        b.iter(|| {
            black_box(ai.get_move(&board, Stone::Black));
        });
    });

    group.finish();
}

fn bench_mcts_playout(c: &mut Criterion) {
    use jungo::ai::{Mcts, MonteCarloAI};

    let mut group = c.benchmark_group("mcts_operations");
    group.sample_size(10); // MCTS is slower, so reduce sample size
    group.measurement_time(std::time::Duration::from_secs(20)); // Increase measurement time

    let board = Board::new(9);

    group.bench_function("MCTS_get_move_9x9_10sims", |b| {
        b.iter(|| {
            let mcts = Mcts::new(10); // Only 10 simulations for benchmarking
            black_box(mcts.get_move(&board, Stone::Black));
        });
    });

    group.bench_function("MonteCarloAI_get_move_9x9_10sims", |b| {
        b.iter(|| {
            let mc = MonteCarloAI::new(10); // Only 10 simulations for benchmarking
            black_box(mc.get_move(&board, Stone::Black));
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_ai_move_selection,
    bench_ai_empty_board,
    bench_ai_dense_board,
    bench_mcts_playout
);
criterion_main!(benches);
