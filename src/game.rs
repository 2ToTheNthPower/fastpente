use std::io::{Read, Write};
use std::fs::File;
use rand::Rng;

use crate::board::Board;
use crate::board::Piece;
use crate::mcts_player::MCTSPlayer;
use crate::random_player::get_piece_by_id;
use crate::random_player::get_piece_id;

// Define struct for game outcomes
pub struct GameOutcome {
    pub is_game_over: bool,
    pub is_draw: bool,
    pub winner: usize,
}


// Implement a game struct that has a board and players
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Game {
    // pub boards: Vec<Board>,
    pub board: Board,
    pub players: Vec<MCTSPlayer>,
    pub player_idx: usize,
}

impl Game {
    // Initialize a new game with a board and num_players players
    pub fn new(size: usize, num_players: usize) -> Game {
        Game {
            // boards: Vec::new(),
            board: Board::new(size),
            // Give every player a different Piece type
            players: (0..num_players).map(|i| MCTSPlayer::new(i, get_piece_by_id(i), 100, 0)).collect(),
            player_idx: 0,
        }
    }

    pub fn reset(&mut self, size: usize, num_players: usize) -> Game {
        Game {
            // boards: Vec::new(),
            board: Board::new(size),
            // Give every player a different Piece type
            players: (0..num_players).map(|i| MCTSPlayer::new(i, get_piece_by_id(i), 100, 0)).collect(),
            player_idx: 0,
        }
    }

    // Define a MCTS rollout function that plays n games from the current game state, and returns each outcome.
    pub fn rollout(&mut self, n: usize) -> (usize, usize, usize){
        let mut winner_0_count = 0;
        let mut winner_1_count = 0;
        let mut is_draw_count = 0;

        for _ in 0..n {
            let mut game = self.clone();
            let (board, reward, done, outcome) = game.run(true);
            if outcome.is_draw {
                is_draw_count += 1;
            } else if outcome.winner == 0 {
                winner_0_count += 1;
            } else if outcome.winner == 1 {
                winner_1_count += 1;
            }
        }
        return (winner_0_count, winner_1_count, is_draw_count)
    }

    // Define a function that checks if the game is over
    pub fn is_game_over(&self, board: &Board, num_in_a_row: usize, num_captured_pairs: usize) -> GameOutcome {
        // Return GameOutcome
        // 1. Check if there are num_in_a_row pieces (of the same color) in a row on diagonals, horizontal, or vertical
        let player = &self.players[self.player_idx];

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
                            winner: player.id,
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
                            winner: player.id,
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
                            winner: player.id, // Ensure get_piece_id is defined
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
                            winner: player.id, // Ensure get_piece_id is defined
                            is_draw: false,
                        };
                    }
                }
            }
        }

        // 2. Check if current player has captured num_captured_pairs of other players pieces
        if player.captured_pairs >= num_captured_pairs {
            // println!("Player {} wins by capturing {} pairs!", player.id, num_captured_pairs);
            return GameOutcome {
                is_game_over: true,
                winner: player.id,
                is_draw: false,
            };
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
                winner: 100,
                is_draw: true,
            };
        } else {
            return GameOutcome {
                is_game_over: false,
                winner: 100,
                is_draw: false,
            };
        }
    }

    // Implement a step function that conforms to the GYM reinforcement learning API standard
    pub fn step(&mut self, action: (usize, usize)) -> (Board, f32, bool, GameOutcome) {
        // 1. Check if the action is valid
        // 2. If the action is valid, apply it to the board
        // 3. Check if the game is over
        // 4. If the game is over, return the board, reward, and done
        // 5. If the game is not over, return the board, 0 reward, and not done
        let player = &mut self.players[self.player_idx];
        let (x, y) = action;

        // self.boards.push(self.board.clone());
        if let Err(e) = player.act(&mut self.board, x, y) {
            println!("MCTSPlayer {} failed to act: {}", 0, e);
        }
        let outcome = self.is_game_over(&self.board, 5, 5);

        if outcome.is_game_over && !outcome.is_draw {
            return (self.board.clone(), outcome.winner as f32, true, outcome);
        }

        (self.board.clone(), 0.0, false, outcome)
    }

    // Write Game to binary file using bincode
    pub fn save(&mut self, file_path: &str) {
        let target: Option<Game> = Some(self.clone());
        let serialized = bincode::serialize(&target).unwrap();
        let mut file = std::fs::File::create(file_path).unwrap();
        file.write_all(&serialized).unwrap();
    }

    // Load Game from binary file using bincode
    pub fn load(file_path: &str) -> Result<Game, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let game: Game = bincode::deserialize(&buffer)?;
        Ok(game)
    }

    // Use step() in a loop to run a game
    pub fn run(mut self, random: bool) -> (Board, f32, bool, GameOutcome) {
        let mut done = false;
        let mut reward = 0.0;
        let mut board = self.board.clone();
        let mut outcome = GameOutcome {
            is_game_over: false,
            winner: 100,
            is_draw: false,
        };
    
        while !done {
            let mut action = (0, 0);
            if !random {
                let player = &self.players[self.player_idx];
                action = player.think(self.clone());
            } else {
                let mut rng = rand::thread_rng();
                let mut valid_actions = self.board.get_moves();
                action = valid_actions[rng.gen_range(0..valid_actions.len())];
            }
        
            let (new_board, new_reward, new_done, new_outcome) = self.step(action);
            board = new_board;
            reward = new_reward;
            done = new_done;
            outcome = new_outcome;
            self.player_idx = (self.player_idx + 1) % self.players.len();
        }
        // println!("Player {} wins!", self.player_idx);
        (board, reward, done, outcome)
    }
}

