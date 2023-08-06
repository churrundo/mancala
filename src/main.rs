use std::fmt;

#[derive(Debug)]
enum Player {
    Player1,
    Player2,
}
struct GameState {
    player1_board: Vec<u32>,
    player2_board: Vec<u32>,
    player1_score: u32,
    player2_score: u32,
    current_player: Player,
}

impl GameState {
    fn make_move(&mut self, pit_index: usize) -> Result<(), &'static str> {
        let (current_board, opponent_board) = match self.current_player {
            Player::Player1 => (&mut self.player1_board, &mut self.player2_board),
            Player::Player2 => (&mut self.player2_board, &mut self.player1_board),
        };

        if current_board[pit_index] == 0 {
            return Err("You cannot choose an empty pit.");
        }

        let mut stones = current_board[pit_index];
        println!("Starting stones: {}", stones);
        current_board[pit_index] = 0;

        let mut current_pit = pit_index + 1;
        let mut on_current_board = true;

        while stones > 0 {
            if on_current_board && current_pit == current_board.len() {
                if let Player::Player1 = self.current_player {
                    self.player1_score += 1;
                } else {
                    self.player2_score += 1;
                }
                stones -= 1;
                if stones == 0 {
                    println!("Last stone landed in Mancala. Player gets another turn.");
                    return Ok(());
                }
                on_current_board = false; // Switch to the opponent's board
                current_pit = 0;
            } else if !on_current_board && current_pit == opponent_board.len() {
                on_current_board = true; // Switch back to the current player's board
                current_pit = 0;
            }

            if on_current_board {
                current_board[current_pit] += 1;
                println!(
                    "Dropped stone in player's pit {}. Remaining stones: {}",
                    current_pit,
                    stones
                );
            } else {
                opponent_board[current_pit] += 1;
                println!(
                    "Dropped stone in opponent's pit {}. Remaining stones: {}",
                    current_pit,
                    stones
                );
            }
            stones -= 1;
            current_pit += 1;
        }

        let last_pit_index = if on_current_board {
            current_pit - 1
        } else {
            opponent_board.len() - current_pit
        };

        println!("Last pit index: {}", last_pit_index);

        if on_current_board && current_board[last_pit_index] == 1 {
            let opponent_pit_index = opponent_board.len() - 1 - last_pit_index;
            if opponent_board[opponent_pit_index] > 0 {
                println!("Capturing {} stones from opponent's pit: {}", opponent_board[opponent_pit_index], opponent_pit_index);
                let captured_stones = opponent_board[opponent_pit_index];
                opponent_board[opponent_pit_index] = 0;
                if let Player::Player1 = self.current_player {
                    self.player1_score += captured_stones + 1;
                    self.player1_board[last_pit_index] = 0;
                } else {
                    self.player2_score += captured_stones + 1;
                    self.player2_board[last_pit_index] = 0;
                }
            } else {
                println!("No stones to capture in opponent's pit: {}", opponent_pit_index);
            }
        }

        println!("Switching player...");
        self.switch_player();
        Ok(())
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };
    }

    fn is_game_over(&self) -> bool {
        // For simplicity, let's say the game ends when either player's current_board is empty
        self.player1_board.iter().all(|&x| x == 0) || self.player2_board.iter().all(|&x| x == 0)
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let player1_board = self.player1_board
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");
        let player2_board = self.player2_board
            .iter()
            .rev()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");

        write!(
            f,
            "Player 2's Mancala:{}\nPlayer 2's current_board:{}\nPlayer 1's current_board:{}\nPlayer 1's Mancala:{}",
            self.player2_score,
            player2_board,
            player1_board,
            self.player1_score
        )
    }
}

fn main() {
    let mut game_state = GameState {
        player1_board: vec![4, 4, 4, 4, 4, 4], // 6 pits for player 1
        player2_board: vec![4, 4, 4, 4, 4, 4], // 6 pits for player 2
        player1_score: 0, // initial score for player 1
        player2_score: 0, // initial score for player 2
        current_player: Player::Player1, // player 1 goes first
    };

    loop {
        println!("{}", game_state); // print the game state at the beginning of each turn

        let chosen_pit: usize;

        loop {
            println!("Current player: {:?}", game_state.current_player);
            println!("Enter the pit index (0-5): ");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse() {
                Ok(n) if n < 6 => {
                    chosen_pit = n;
                    break;
                }
                _ => {
                    println!("Invalid input. Please enter a number between 0 and 5.");
                }
            }
        }

        match game_state.make_move(chosen_pit) {
            Ok(()) => {
                // The move was successful
            }
            Err(msg) => {
                // There was an error
                println!("{}", msg);
                continue; // skips to the next iteration, prompting the player again
            }
        }

        // Check if game over condition is met
        if game_state.is_game_over() {
            println!("Game over!");
            break;
        }
    }
}
