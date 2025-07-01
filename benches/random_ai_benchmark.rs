use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jungo::ai::RandomAI;
use jungo::board::{Board, Stone};
use jungo::player::Player;

fn benchmark_random_ai_empty_board(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_ai_empty");

    for size in [5, 9, 19].iter() {
        group.bench_with_input(format!("{}x{}", size, size), size, |b, &size| {
            let board = Board::new(size);
            let ai = RandomAI::new();
            b.iter(|| black_box(ai.get_move(&board, Stone::Black)));
        });
    }

    group.finish();
}

fn benchmark_random_ai_midgame(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_ai_midgame");

    for size in [5, 9].iter() {
        group.bench_with_input(format!("{}x{}", size, size), size, |b, &size| {
            let mut board = Board::new(size);
            // Fill about 40% of the board
            let positions_to_fill = (size * size * 2 / 5) as usize;
            let mut filled = 0;
            let mut stone = Stone::Black;

            for y in 0..size {
                for x in 0..size {
                    if filled >= positions_to_fill {
                        break;
                    }
                    if (x + y) % 3 != 0 {
                        // Create some pattern
                        if board.place_stone(x, y, stone).is_ok() {
                            filled += 1;
                            stone = stone.opposite();
                        }
                    }
                }
            }

            let ai = RandomAI::new();
            b.iter(|| black_box(ai.get_move(&board, Stone::Black)));
        });
    }

    group.finish();
}

fn benchmark_random_ai_endgame(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_ai_endgame");

    group.bench_function("5x5_few_moves", |b| {
        let mut board = Board::new(5);
        // Fill most of the board, leaving only a few valid moves
        for y in 0..5 {
            for x in 0..5 {
                if !((x == 2 && y == 2) || (x == 1 && y == 1) || (x == 3 && y == 3)) {
                    let stone = if (x + y) % 2 == 0 {
                        Stone::Black
                    } else {
                        Stone::White
                    };
                    let _ = board.place_stone(x, y, stone);
                }
            }
        }

        let ai = RandomAI::new();
        b.iter(|| black_box(ai.get_move(&board, Stone::Black)));
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_random_ai_empty_board,
    benchmark_random_ai_midgame,
    benchmark_random_ai_endgame
);
criterion_main!(benches);
