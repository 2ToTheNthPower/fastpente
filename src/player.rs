use rand::Rng;
use crate::random_player::RandomPlayer;
use crate::mcts_player::MCTSPlayer;

use crate::{board::{Board, Piece}, game::Game};

pub trait PlayerBehavior {
    fn act(&mut self, board: &mut Board, x: usize, y: usize) -> Result<(), String>;
    fn think(&self, game: &Game) -> (usize, usize); 
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn get_capture_count(&self) -> usize;
    fn set_capture_count(&mut self, count: usize);
}

impl PlayerBehavior for RandomPlayer {
    fn act(&mut self, board: &mut Board, x: usize, y: usize) -> Result<(), String> {
        if x >= board.size || y >= board.size {
            return Err("Position out of bounds".to_string());
        }
        if board.grid[x][y] != Piece::Empty {
            return Err("Position already occupied".to_string());
        }
        board.grid[x][y] = self.piece_type.clone();
        // Capture logic
        self.capture(board, x, y);

        Ok(())
    }

    fn think(&self, mut game: &Game) -> (usize, usize) {
        // Choose random unoccupied position
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..game.board.size);
            let y = rng.gen_range(0..game.board.size);
            if game.board.grid[x][y] == Piece::Empty {
                return (x, y);
            }
        }
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn get_capture_count(&self) -> usize {
        self.captured_pairs
    }

    fn set_capture_count(&mut self, count: usize) {
        self.captured_pairs = count;
    }
}


impl PlayerBehavior for MCTSPlayer {
    fn act(&mut self, board: &mut Board, x: usize, y: usize) -> Result<(), String> {
        if x >= board.size || y >= board.size {
            return Err("Position out of bounds".to_string());
        }
        if board.grid[x][y] != Piece::Empty {
            return Err("Position already occupied".to_string());
        }
        board.grid[x][y] = self.piece_type.clone();
        // Capture logic
        self.capture(board, x, y);

        Ok(())
    }

    // Define think function that performs MCTS rollout
    fn think(&self, mut game: &Game) -> (usize, usize) {
        // Choose random unoccupied position
        let mut valid_actions = game.board.get_moves();

        // For every valid action, perform a rollout
        let mut rng = rand::thread_rng();
        let mut action_scores = vec![0; valid_actions.len()];

        for (i, action) in valid_actions.iter().enumerate() {
            let mut total_score = 0;
            let (winner_0_count, winner_1_count, is_draw_count) = game.rollout(self.num_rollouts);
            if self.id == 0 {
                total_score = winner_0_count / self.num_rollouts;
            } else if self.id == 1 {
                total_score = winner_1_count / self.num_rollouts;
            }
            action_scores[i] = total_score;
        }
        // Assuming action_scores is a Vec<usize> and valid_actions is a Vec of the same length
        let max_index = action_scores.iter()
        .enumerate() // attach indices to the elements
        .max_by_key(|&(_idx, &val)| val) // find the maximum value with its index
        .map(|(idx, _)| idx) // extract the index
        .unwrap(); // handle the None case appropriately for your context

        return valid_actions[max_index]; // use the index to return the corresponding element from valid_actions

    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn get_capture_count(&self) -> usize {
        self.captured_pairs
    }

    fn set_capture_count(&mut self, count: usize) {
        self.captured_pairs = count;
    }
}