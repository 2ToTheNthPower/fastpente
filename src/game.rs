use crate::board::Board;
use crate::board::Piece;
use crate::player::RandomPlayer;
use crate::player::get_piece_by_id;
use crate::player::get_piece_id;

// Define struct for game outcomes
pub struct GameOutcome {
    pub is_game_over: bool,
    pub is_draw: bool,
    pub winner: Option<usize>,
}


// Implement a game struct that has a board and players
pub struct Game {
    pub board: Board,
    pub players: Vec<RandomPlayer>,
}

impl Game {
    // Initialize a new game with a board and num_players players
    pub fn new(size: usize, num_players: usize) -> Game {
        Game {
            board: Board::new(size),
            // Give every player a different Piece type
            players: (0..num_players).map(|i| RandomPlayer::new(i, get_piece_by_id(i))).collect(),
        }
    }

    pub fn is_game_over(&self, board: &Board, num_in_a_row: usize, num_captured_pairs: usize) -> GameOutcome {
        // Return GameOutcome
        // 1. Check if there are num_in_a_row pieces (of the same color) in a row on diagonals, horizontal, or vertical

        for row in 0..board.size {
            for col in 0..board.size {
                let piece = &board.grid[row][col];
                if *piece == Piece::Empty {
                    continue;
                }
                // Check horizontal
                if col + num_in_a_row <= board.size {
                    let mut is_win = true;
                    for i in 1..num_in_a_row {
                        if board.grid[row][col + i] != *piece {
                            is_win = false;
                            break;
                        }
                    }
                    if is_win {
                        return GameOutcome {
                            is_game_over: true,
                            winner: Some(get_piece_id(piece)),
                            is_draw: false,
                        };
                    }
                }
                // Check vertical
                if row + num_in_a_row <= board.size {
                    let mut is_win = true;
                    for i in 1..num_in_a_row {
                        if board.grid[row + i][col] != *piece {
                            is_win = false;
                            break;
                        }
                    }
                    if is_win {
                        return GameOutcome {
                            is_game_over: true,
                            winner: Some(get_piece_id(piece)),
                            is_draw: false,
                        };
                    }
                }
                // Check diagonal
                
                if col + num_in_a_row <= board.size && row + num_in_a_row <= board.size {
                    let mut is_win = true;
                    for i in 1..num_in_a_row {
                        if board.grid[row + i][col + i] != *piece {
                            is_win = false;
                            break;
                        }
                    }
                    if is_win {
                        return GameOutcome {
                            is_game_over: true,
                            winner: Some(get_piece_id(piece)), // Ensure get_piece_id is defined
                            is_draw: false,
                        };
                    }
                }
                // Check anti-diagonal
                if col + 1 >= num_in_a_row && row + num_in_a_row <= board.size {
                    let mut is_win = true;
                    for i in 1..num_in_a_row {
                        if board.grid[row + i][col - i] != *piece {
                            is_win = false;
                            break;
                        }
                    }
                    if is_win {
                        return GameOutcome {
                            is_game_over: true,
                            winner: Some(get_piece_id(piece)), // Ensure get_piece_id is defined
                            is_draw: false,
                        };
                    }
                }
            }
        }

        // 2. Check if any player has captured num_captured_pairs of other players pieces
        for player in &self.players {
            if player.captured_pairs >= num_captured_pairs {
                println!("Player {} wins by capturing {} pairs!", player.id, num_captured_pairs);
                return GameOutcome {
                    is_game_over: true,
                    winner: Some(get_piece_id(&player.piece_type)),
                    is_draw: false,
                };
            }
        }

        // 3. Check if the board is full
        let mut is_full = true;
        for row in 0..board.size {
            for col in 0..board.size {
                if board.grid[row][col] == Piece::Empty {
                    is_full = false;
                    break;
                }
            }
        }

        if is_full {
            return GameOutcome {
                is_game_over: true,
                winner: None,
                is_draw: true,
            };
        } else {
            return GameOutcome {
                is_game_over: false,
                winner: None,
                is_draw: false,
            };
        }
    }

    // Run the game loop
    pub fn run(mut self) -> GameOutcome {
        // Implement the game loop here
        // 1. Print the board
        // 2. Ask the current player to plan a move
        // 3. Ask the current player to act on the board
        // 4. Check if the game is over
        // 5. If not, go to step 1 with the next player
        // 6. If yes, print the board and declare the winner
        let mut current_player = 0;
        loop {
            // println!("{}", self.board);
            let (x, y) = self.players[current_player].think(&self.board);
            if let Err(e) = self.players[current_player].act(&mut self.board, x, y) {
                println!("RandomPlayer {} failed to act: {}", current_player, e);
            }
            let outcome = self.is_game_over(&self.board, 5, 5);
            if outcome.is_game_over {
                // println!("{}", self.board);
                println!("RandomPlayer {} wins!", current_player);
                return outcome;
            }
            current_player = (current_player + 1) % self.players.len();
        }
    }
}
