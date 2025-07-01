use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use jungo::ai::{RandomAI, RandomAIV2, RandomAIV3, RandomAIV4};
use jungo::board::{Board, Stone};
use jungo::player::Player;

fn setup_boards() -> Vec<(&'static str, Board)> {
    let mut boards = Vec::new();

    // Empty 9x9
    boards.push(("empty_9x9", Board::new(9)));

    // Midgame 9x9 (40% filled)
    let mut midgame = Board::new(9);
    for i in 0..32 {
        let x = (i * 7 + i / 3) % 9;
        let y = (i * 11 + i / 5) % 9;
        let stone = if i % 2 == 0 {
            Stone::Black
        } else {
            Stone::White
        };
        let _ = midgame.place_stone(x, y, stone);
    }
    boards.push(("midgame_9x9", midgame));

    // Endgame 9x9 (80% filled)
    let mut endgame = Board::new(9);
    for y in 0..9 {
        for x in 0..9 {
            if (x + y) % 5 != 0 {
                let stone = if (x + y) % 2 == 0 {
                    Stone::Black
                } else {
                    Stone::White
                };
                let _ = endgame.place_stone(x, y, stone);
            }
        }
    }
    boards.push(("endgame_9x9", endgame));

    boards
}

fn bench_random_ai_quick(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_ai_9x9");
    group.sample_size(50); // Smaller sample size for quicker results

    let boards = setup_boards();

    for (scenario, board) in &boards {
        // Original
        group.bench_with_input(BenchmarkId::new("Original", scenario), board, |b, board| {
            let ai = RandomAI::new();
            b.iter(|| {
                black_box(ai.get_move(board, Stone::Black));
            });
        });

        // V2 Early Exit
        group.bench_with_input(
            BenchmarkId::new("V2_EarlyExit", scenario),
            board,
            |b, board| {
                let ai = RandomAIV2::new();
                b.iter(|| {
                    black_box(ai.get_move(board, Stone::Black));
                });
            },
        );

        // V3 Reservoir
        group.bench_with_input(
            BenchmarkId::new("V3_Reservoir", scenario),
            board,
            |b, board| {
                let ai = RandomAIV3::new();
                b.iter(|| {
                    black_box(ai.get_move(board, Stone::Black));
                });
            },
        );

        // V4 PreFilter
        group.bench_with_input(
            BenchmarkId::new("V4_PreFilter", scenario),
            board,
            |b, board| {
                let ai = RandomAIV4::new();
                b.iter(|| {
                    black_box(ai.get_move(board, Stone::Black));
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_random_ai_quick);
criterion_main!(benches);
