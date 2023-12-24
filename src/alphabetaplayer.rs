use crate::board::Board;
use crate::board::Piece;
use rand::Rng;
use rand::seq::SliceRandom;

pub struct AlphaBetaPlayer {
    pub id: usize, // or some other identifier
    pub piece_type: Piece,
    pub captured_pairs: usize,
}

pub fn get_piece_by_id(id: usize) -> Piece {
    match id % 4 {
        0 => Piece::Black,
        1 => Piece::White,
        2 => Piece::Red,
        3 => Piece::Green,
        _ => unreachable!(),
    }
}

pub fn get_piece_id(piece: &Piece) -> usize {
    match *piece {
        Piece::Black => 0,
        Piece::White => 1,
        Piece::Red   => 2,
        Piece::Green => 3,
        _ => unreachable!(),
    }
}

impl AlphaBetaPlayer {
    pub fn new(id: usize, piece_type: Piece) -> AlphaBetaPlayer {
        AlphaBetaPlayer { id, piece_type, captured_pairs: 0 }
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

        // println!("AlphaBetaPlayer {} placed a piece at ({}, {})", self.id, x, y);

        Ok(())
    }

    pub fn minimax(&self, board: &mut Board, depth: isize, mut alpha: isize, mut beta: isize, maximizing_player: bool) -> isize {
        // Base case
        if depth == 0 {
            return 0;
        }

        // Recursive case
        if maximizing_player {
            let mut best_score = std::isize::MIN;
            let mut moves = board.get_moves();
            let mut rng = rand::thread_rng();
            moves.shuffle(&mut rng);

            for (x, y) in moves {
                let mut board_copy = board.clone();
                board_copy.grid[x][y] = self.piece_type.clone();
                let score = self.minimax(&mut board_copy, depth - 1, alpha, beta, false);
                best_score = best_score.max(score);
                alpha = alpha.max(score);
                if beta <= alpha {
                    break;
                }
            }

            best_score
        } else {
            let mut best_score = std::isize::MAX;
            let mut moves = board.get_moves();
            let mut rng = rand::thread_rng();
            moves.shuffle(&mut rng);

            for (x, y) in moves {
                let mut board_copy = board.clone();
                board_copy.grid[x][y] = self.piece_type.clone();
                let score = self.minimax(&mut board_copy, depth - 1, alpha, beta, true);
                best_score = best_score.min(score);
                beta = beta.min(score);
                if beta <= alpha {
                    break;
                }
            }

            best_score
        }
    }

    pub fn think(&self, board: &Board) -> (usize, usize) {
        // Perform alpha beta search
        let mut best_move = (0, 0);
        let mut best_score = std::isize::MIN;
        let mut alpha = std::isize::MIN;
        let mut beta = std::isize::MAX;
        let mut depth = 3;
        let mut rng = rand::thread_rng();
        let mut moves = board.get_moves();
        moves.shuffle(&mut rng);

        for (x, y) in moves {
            let mut board_copy = board.clone();
            board_copy.grid[x][y] = self.piece_type.clone();
            let score = self.minimax(&mut board_copy, depth, alpha, beta, false);
            if score > best_score {
                best_score = score;
                best_move = (x, y);
            }
            alpha = alpha.max(score);
        }

        best_move
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_act() {
        let mut board = Board::new(3);
        let mut player = AlphaBetaPlayer::new(0, Piece::Black);
        assert_eq!(player.act(&mut board, 0, 0), Ok(()));
        assert_eq!(player.act(&mut board, 0, 0), Err("Position already occupied".to_string()));
        assert_eq!(player.act(&mut board, 3, 0), Err("Position out of bounds".to_string()));
    }

    #[test]
    fn test_player_capture() {
        let mut board = Board::new(5);
        let mut player_0 = AlphaBetaPlayer::new(0, Piece::Black);
        let mut player_1 = AlphaBetaPlayer::new(1, Piece::White);

        // Prepare the board for capturing test
        assert_eq!(player_0.act(&mut board, 1, 1), Ok(()));
        assert_eq!(player_1.act(&mut board, 1, 0), Ok(()));
        assert_eq!(player_0.act(&mut board, 1, 2), Ok(()));

        // Assuming player_1 can capture pieces at these coordinates
        player_1.capture(&mut board, 1, 3); // AlphaBetaPlayer 1 attempts to capture

        // Assertions about captured pairs or board state after capture
        // These would need to be adjusted based on how your game rules define a "capture"
        assert_eq!(player_1.captured_pairs, 1);
        assert_eq!(board.grid[1][1], Piece::Empty); // Assuming this piece would be captured
        assert_eq!(board.grid[1][2], Piece::Empty); // Assuming this piece would be captured
    }
}
