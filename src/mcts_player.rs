use crate::{board::{Board, Piece}, game::Game};

// Implement player that performs MCTS rollout

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct MCTSPlayer {
    pub piece_type: Piece,
    pub id: usize,
    pub num_rollouts: usize,
    pub captured_pairs: usize,
    }

impl MCTSPlayer {
    pub fn new(id: usize, piece_type: Piece, num_rollouts: usize, captured_pairs: usize) -> MCTSPlayer {
        MCTSPlayer { id, piece_type, num_rollouts, captured_pairs }
    }

    pub fn act(&mut self, board: &mut Board, x: usize, y: usize) -> Result<(), String> {
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
    pub fn think(&self, mut game: Game) -> (usize, usize) {
        // Choose random unoccupied position
        let mut valid_actions = game.board.get_moves();

        // For every valid action, perform a rollou
        let mut best_score = 0.0;
        let mut best_action = valid_actions[0];

        for (i, action) in valid_actions.iter().enumerate() {
            let mut total_score: f32 = 0.0;
            let (winner_0_count, winner_1_count, is_draw_count) = game.rollout(self.num_rollouts);
            // println!("{} {} {}", winner_0_count, winner_1_count, is_draw_count);
            if self.id == 0 {
                total_score = (winner_0_count as f32) / (self.num_rollouts as f32);
            } else if self.id == 1 {
                total_score = winner_1_count as f32 / self.num_rollouts as f32;
            }
            if total_score > best_score {
                // println!("{} {}", total_score, best_score);
                best_score = total_score;
                best_action = *action;
            }
        }
        return best_action;
    }

    pub fn owns_piece(&self, board: &Board, x: usize, y: usize) -> bool {
        board.grid[x][y] == self.piece_type
    }

    pub fn capture(&mut self, board: &mut Board, x: usize, y: usize) {
        let movement_vectors: [(isize, isize); 8] = [
            (0, 1), (0, -1), (1, 0), (-1, 0),
            (1, 1), (-1, -1), (1, -1), (-1, 1),
        ];

        let x_isize = x as isize;  // Convert to isize for safe arithmetic
        let y_isize = y as isize;

        for &movement_vector in &movement_vectors {
            let first_x = x_isize.checked_add(movement_vector.0);
            let first_y = y_isize.checked_add(movement_vector.1);
            let second_x = x_isize.checked_add(2 * movement_vector.0);
            let second_y = y_isize.checked_add(2 * movement_vector.1);
            let pair_x = x_isize.checked_add(3 * movement_vector.0);  // Position to check for a matching piece
            let pair_y = y_isize.checked_add(3 * movement_vector.1);

            if let (Some(first_x), Some(first_y), Some(pair_x), Some(pair_y), Some(second_x), Some(second_y)) = (first_x, first_y, pair_x, pair_y, second_x, second_y) {
                if pair_x >= 0 && pair_y >= 0 &&
                    (pair_x as usize) < board.size && 
                    (pair_y as usize) < board.size {

                    let first_x = first_x as usize;
                    let first_y = first_y as usize;
                    let second_x = second_x as usize;
                    let second_y = second_y as usize;
                    let pair_x = pair_x as usize;
                    let pair_y = pair_y as usize;

                    // Capture logic
                    if self.owns_piece(board, pair_x, pair_y) && // Check if player owns the piece at the pair position
                        board.grid[first_x][first_y] != Piece::Empty && // Make sure it's not empty
                        board.grid[second_x][second_y] != Piece::Empty && // Capture the next piece
                        board.grid[first_x][first_y] != self.piece_type && // Check if the next piece is an opponent's piece
                        board.grid[second_x][second_y] == board.grid[first_x][first_y] {

                        // Assuming a capture scenario - sandwiching one piece
                        self.captured_pairs += 1; // Increment captured pairs
                        // Clear the captured piece(s) from the board
                        board.grid[first_x][first_y] = Piece::Empty; // Capture the next piece
                        board.grid[second_x][second_y] = Piece::Empty; // Capture the next piece

                        // Extend this logic if your game involves capturing multiple pieces per move
                    }
                }
            }
        }
    }
}