use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use jungo::board::{Board, Stone};

fn bench_is_valid_move(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_valid_move");

    for size in [9, 13, 19].iter() {
        group.bench_with_input(BenchmarkId::new("Board", size), size, |b, &size| {
            let board = Board::new(size);
            let positions: Vec<(usize, usize)> = (0..size)
                .flat_map(|y| (0..size).map(move |x| (x, y)))
                .collect();

            b.iter(|| {
                for &(x, y) in &positions {
                    black_box(board.is_valid_move(x, y, Stone::Black));
                }
            });
        });
    }

    group.finish();
}

fn bench_place_stone(c: &mut Criterion) {
    let mut group = c.benchmark_group("place_stone");

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
    ];

    group.bench_function("Board_19x19", |b| {
        b.iter(|| {
            let mut board = Board::new(19);
            let mut stone = Stone::Black;
            for &(x, y) in &moves {
                let _ = board.place_stone(x, y, stone);
                stone = stone.opposite();
            }
        });
    });

    group.finish();
}

fn bench_get_operation(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");

    group.bench_function("Board_19x19", |b| {
        let board = Board::new(19);
        let positions: Vec<(usize, usize)> =
            (0..19).flat_map(|y| (0..19).map(move |x| (x, y))).collect();

        b.iter(|| {
            for &(x, y) in &positions {
                black_box(board.get(x, y));
            }
        });
    });

    group.finish();
}

fn bench_is_eye(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_eye");

    // Setup a board with some stones to make eye detection more realistic
    let mut board_regular = Board::new(19);

    // Create some eye-like patterns
    let setup_moves = vec![
        (5, 5, Stone::Black),
        (6, 5, Stone::Black),
        (5, 6, Stone::Black),
        (10, 10, Stone::White),
        (11, 10, Stone::White),
        (10, 11, Stone::White),
    ];

    for &(x, y, stone) in &setup_moves {
        let _ = board_regular.place_stone(x, y, stone);
    }

    group.bench_function("Board_19x19", |b| {
        let positions: Vec<(usize, usize)> =
            (0..19).flat_map(|y| (0..19).map(move |x| (x, y))).collect();

        b.iter(|| {
            for &(x, y) in &positions {
                black_box(board_regular.is_eye(x, y, Stone::Black));
            }
        });
    });

    group.finish();
}

fn bench_full_game_playout(c: &mut Criterion) {
    use jungo::ai::RandomAI;
    use jungo::player::Player;

    let mut group = c.benchmark_group("full_game_playout");
    group.sample_size(20); // Reduce sample size for longer benchmarks

    group.bench_function("RandomAI_9x9", |b| {
        b.iter(|| {
            let mut board = Board::new(9);
            let ai1 = RandomAI::new();
            let ai2 = RandomAI::new();
            let mut current_turn = Stone::Black;
            let mut consecutive_passes = 0;

            loop {
                let ai: &dyn Player = match current_turn {
                    Stone::Black => &ai1,
                    Stone::White => &ai2,
                };

                match ai.get_move(&board, current_turn) {
                    Some((x, y)) => {
                        if board.place_stone(x, y, current_turn).is_ok() {
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
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_is_valid_move,
    bench_place_stone,
    bench_get_operation,
    bench_is_eye,
    bench_full_game_playout
);
criterion_main!(benches);
