use jungo::board::{Board, Stone};

fn main() {
    println!("Testing Eye detection implementation");

    // Test 1: Simple center eye
    println!("\n=== Test 1: Simple center eye ===");
    let mut board = Board::new(5);
    board.place_stone(1, 0, Stone::Black).unwrap();
    board.place_stone(0, 1, Stone::Black).unwrap();
    board.place_stone(2, 1, Stone::Black).unwrap();
    board.place_stone(1, 2, Stone::Black).unwrap();
    println!("{}", board);
    println!(
        "Is (1,1) an eye for Black? {}",
        board.is_eye(1, 1, Stone::Black)
    );
    println!(
        "Is (1,1) an eye for White? {}",
        board.is_eye(1, 1, Stone::White)
    );

    // Test 2: Corner eye
    println!("\n=== Test 2: Corner eye ===");
    let mut board = Board::new(5);
    board.place_stone(1, 0, Stone::White).unwrap();
    board.place_stone(0, 1, Stone::White).unwrap();
    board.place_stone(1, 1, Stone::White).unwrap(); // Diagonal must be present
    println!("{}", board);
    println!(
        "Is (0,0) an eye for White? {}",
        board.is_eye(0, 0, Stone::White)
    );

    // Test 3: Corner without diagonal - not an eye
    println!("\n=== Test 3: Corner without diagonal ===");
    let mut board = Board::new(5);
    board.place_stone(1, 0, Stone::White).unwrap();
    board.place_stone(0, 1, Stone::White).unwrap();
    // No diagonal stone
    println!("{}", board);
    println!(
        "Is (0,0) an eye for White? {}",
        board.is_eye(0, 0, Stone::White)
    );

    // Test 4: False eye (too many enemy diagonals)
    println!("\n=== Test 4: False eye ===");
    let mut board = Board::new(5);
    // Create surrounding stones
    board.place_stone(1, 0, Stone::Black).unwrap();
    board.place_stone(0, 1, Stone::Black).unwrap();
    board.place_stone(2, 1, Stone::Black).unwrap();
    board.place_stone(1, 2, Stone::Black).unwrap();
    // Add enemy stones on diagonals
    board.place_stone(0, 0, Stone::White).unwrap();
    board.place_stone(2, 0, Stone::White).unwrap();
    println!("{}", board);
    println!(
        "Is (1,1) an eye for Black? {}",
        board.is_eye(1, 1, Stone::Black)
    );

    // Test 5: Edge eye
    println!("\n=== Test 5: Edge eye ===");
    let mut board = Board::new(5);
    board.place_stone(0, 0, Stone::Black).unwrap();
    board.place_stone(2, 0, Stone::Black).unwrap();
    board.place_stone(1, 1, Stone::Black).unwrap();
    println!("{}", board);
    println!(
        "Is (1,0) an eye for Black? {}",
        board.is_eye(1, 0, Stone::Black)
    );

    // Test 6: Count eyes
    println!("\n=== Test 6: Count total eyes ===");
    let mut board = Board::new(5);
    // Create multiple eyes
    board.place_stone(1, 0, Stone::Black).unwrap();
    board.place_stone(0, 1, Stone::Black).unwrap();
    board.place_stone(2, 1, Stone::Black).unwrap();
    board.place_stone(1, 2, Stone::Black).unwrap();

    board.place_stone(3, 0, Stone::Black).unwrap();
    board.place_stone(4, 1, Stone::Black).unwrap();
    board.place_stone(3, 2, Stone::Black).unwrap();

    println!("{}", board);
    println!(
        "Total eyes for Black: {}",
        board.count_eyes_for_color(Stone::Black)
    );

    // Test 7: The problematic case from test_not_eye_when_not_fully_surrounded
    println!("\n=== Test 7: Edge case from failing test ===");
    let mut board = Board::new(5);
    // Place white stones
    board.place_stone(1, 0, Stone::White).unwrap(); // B5
    board.place_stone(2, 0, Stone::White).unwrap(); // C5
    board.place_stone(4, 0, Stone::White).unwrap(); // E5
    board.place_stone(0, 1, Stone::White).unwrap(); // A4
    board.place_stone(2, 1, Stone::White).unwrap(); // C4
    board.place_stone(3, 1, Stone::White).unwrap(); // D4
    board.place_stone(4, 1, Stone::White).unwrap(); // E4

    println!("{}", board);
    println!(
        "Is (0,0) an eye for White? {}",
        board.is_eye(0, 0, Stone::White)
    ); // A5
    println!(
        "Is (3,0) an eye for White? {}",
        board.is_eye(3, 0, Stone::White)
    ); // D5
    println!(
        "Is (1,1) an eye for White? {}",
        board.is_eye(1, 1, Stone::White)
    ); // B4

    // Add the diagonal to make A5 an eye
    board.place_stone(1, 1, Stone::White).unwrap(); // B4
    println!("\nAfter adding B4:");
    println!("{}", board);
    println!(
        "Is (0,0) an eye for White? {}",
        board.is_eye(0, 0, Stone::White)
    ); // A5
}
