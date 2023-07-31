struct GameState {
    player1_board: Vec<u32>,
    player2_board: Vec<u32>,
    player1_score: u32,
    player2_score: u32,
}

fn main() {
    let mut game_state = GameState {
        player1_board: vec![4, 4, 4, 4, 4, 4],  // 6 pits for player 1
        player2_board: vec![4, 4, 4, 4, 4, 4],  // 6 pits for player 2
        player1_score: 0,  // initial score for player 1
        player2_score: 0,  // initial score for player 2
    };

    println!("Hello, world!");
}
