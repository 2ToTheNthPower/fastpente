use crate::board::Board;
use crate::board::Piece;
use rand::Rng;

pub struct Player {
    pub id: usize, // or some other identifier
    pub piece_type: Piece,
    pub captured_pairs: usize,
}

pub fn get_piece_by_id(id: usize) -> Piece {
    match id{ // using % 4 for cycling through 4 player types
        0 => Piece::Black,
        1 => Piece::White,
        2 => Piece::Red,
        3 => Piece::Green,
        _ => unreachable!(), // This is theoretically unreachable with % 4
    }
}

pub fn get_piece_id(piece: Piece) -> usize {
    match piece {
        Piece::Black => 0,
        Piece::White => 1,
        Piece::Red   => 2,
        Piece::Green => 3,
        _ => unreachable!(), // This is theoretically unreachable with % 4
    }
}

impl Player {
    pub fn new(id: usize, piece_type: Piece) -> Player {
        Player { id, piece_type , captured_pairs: 0}
    }

    // A method for the player to place a piece on the board
    pub fn act(&self, board: &mut Board, x: usize, y: usize) -> Result<(), String> {
        if x >= board.size || y >= board.size {
            return Err("Position out of bounds".to_string());
        }
        if board.grid[x][y] != Piece::Empty {
            return Err("Position already occupied".to_string());
        }
        board.grid[x][y] = self.piece_type.clone();
        Ok(())
    }

    // A method for the player planning a move
    pub fn think(&self, board: &Board) -> (usize, usize) {
        // implement logic here

        // for now, just return a random position
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..board.size);
        let y = rng.gen_range(0..board.size);
        (x, y)
    }

    // A method for incrementing the number of captured pairs by n
    pub fn capture(mut self, n: usize) {
        self.captured_pairs += n;
    }

}

// Test code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_act() {
        let mut board = Board::new(3);
        let player = Player::new(0, Piece::Black);
        assert_eq!(player.act(mut board, 0, 0), Ok(()));
        assert_eq!(player.act(mut board, 0, 0), Err("Position already occupied".to_string()));
        assert_eq!(player.act(mut board, 3, 0), Err("Position out of bounds".to_string()));
    }
}

