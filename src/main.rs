use std::fmt;

struct GameState {
    player1_board: Vec<u32>,
    player2_board: Vec<u32>,
    player1_score: u32,
    player2_score: u32,
}

fn main() {
    let game_state = GameState {
        player1_board: vec![4, 4, 4, 4, 4, 4], // 6 pits for player 1
        player2_board: vec![4, 4, 4, 4, 4, 4], // 6 pits for player 2
        player1_score: 0, // initial score for player 1
        player2_score: 0, // initial score for player 2
    };

    impl fmt::Display for GameState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let player1_board = self.player1_board
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ");
            let player2_board = self.player2_board
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ");

            write!(
                f,
                "Player 2's Mancala:{}\nPlayer 2's board:{}\nPlayer 1's board:{}\nPlayer 1's Mancala:{}",
                self.player2_score,
                player2_board,
                player1_board,
                self.player1_score
            )
        }
        
    }
    println!("{}", game_state);
}
