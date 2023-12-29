use pyo3::prelude::*;
use ndarray::Array2;


#[pyclass]
pub struct Pente {
    pub board: Array2<i8>,
    pub size: u8,
    pub current_player: i8,
    pub player_1_pairs: u8,
    pub player_2_pairs: u8,
}

#[pymethods]
impl Pente {
    #[new]
    pub fn new() -> Pente {
        Pente {
            board: Array2::zeros((19, 19)),
            size: 19,
            current_player: 1,
            player_1_pairs: 0,
            player_2_pairs: 0,
        }
    }

    pub fn reset(&mut self) -> Pente{
        Pente {
            board: Array2::zeros((19, 19)),
            size: 19,
            current_player: 1,
            player_1_pairs: 0,
            player_2_pairs: 0,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> i8 {
        self.board[[row, col]]
    }

    pub fn set(&mut self, row: usize, col: usize, val: i8) {
        self.board[[row, col]] = val;
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn is_full(&self) -> bool {
        self.board.iter().all(|&x| x != 0)
    }

    pub fn is_on_board(&self, row: isize, col: isize) -> bool {
        row < self.size as isize && col < self.size as isize && row >= 0 && col >= 0
    }

    pub fn is_valid_action(&self, row: isize, col: isize) -> bool {
        self.is_on_board(row, col) && self.board[[row as usize, col as usize]] == 0
    }

    pub fn get_pair_count(&self, player: i8) -> u8 {
        if player == -1 {
            self.player_1_pairs
        } else {
            self.player_2_pairs
        }
    }

    pub fn capture(&mut self, row: usize, col: usize) {
        // Get the current player
        let player = self.current_player;

        // Get every motion vector (up, down, right, left, up-right, up-left, down-right, down-left)
        let mut motion_vectors: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, 1), (1, -1), (-1, -1)];

        // Check if third stone away along each motion vector is the same as the current player
        for (row_offset, col_offset) in motion_vectors {
            let opposite_row = row as isize + row_offset * 3;
            let opposite_col = col as isize + col_offset * 3;

            // Check if the opposite stone is on the board
            if self.is_on_board(opposite_row, opposite_col) {
                // Check if the opposite stone is the same as the current player
                if self.board[[opposite_row as usize, opposite_col as usize]] == player {
                    // Check if the two stones in between are the other player
                    let first_row = row as isize + row_offset;
                    let first_col = col as isize + col_offset;

                    if self.is_on_board(first_row, first_col) && self.board[[first_row as usize, first_col as usize]] == -player {
                        let second_row = row as isize + row_offset*2;
                        let second_col = col as isize + col_offset*2;

                        if self.is_on_board(second_row, second_col) && self.board[[second_row as usize, second_col as usize]] == -player {
                            // If so, capture the stones
                            if player == -1 {
                                self.player_1_pairs += 1;
                            } else if player == 1 {
                                self.player_2_pairs += 1;
                            }
                            // Remove captured pieces
                            self.board[[first_row as usize, first_col as usize]] = 0; 
                            self.board[[second_row as usize, second_col as usize]] = 0;
                        }
                    }
                }
            }
        }
    }

    pub fn place(&mut self, row: usize, col: usize) {
        // Place the stone
        if self.is_valid_action(row as isize, col as isize) {
            self.board[[row, col]] = self.current_player;
        } else {
            panic!("Invalid action");
        }
    }

    // Row, Col here is the last action taken.  Needed to speed up win condition checks.
    pub fn is_done(&self, row: usize, col: usize) -> bool {
        if self.player_1_pairs >= 5 {
            println!("Player 1 wins!");
            return true;
        } else if self.player_2_pairs >= 5 {
            println!("Player 2 wins!");
            return true;
        } else if self.is_full() {
            println!("Draw!");
            return true;
        } else {
            // Check if the current player has five in a row
            let mut motion_vectors: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (1, 1), (-1, 1)];

            for (row_offset, col_offset) in motion_vectors {
                let mut consecutive = 0;
                for i in -5..5 {
                    let cur_row = row as isize + row_offset * i;
                    let cur_col = col as isize + col_offset * i;

                    if (consecutive >= i - 1) && self.is_on_board(cur_row, cur_col) && self.board[[cur_row as usize, cur_col as usize]] == self.current_player {
                        consecutive += 1;
                        if consecutive >= 5 {
                            println!("Player {} wins!", self.current_player);
                            return true;
                        }
                    } else {
                        // Reset consecutive count to zero
                        consecutive = 0;
                    }
                }
            }
        }
        return false;
    }

    pub fn get_valid_actions(&self) -> Vec<(usize, usize)> {
        let mut valid_actions: Vec<(usize, usize)> = Vec::new();

        for row in 0..self.size as usize {
            for col in 0..self.size as usize {
                if self.is_valid_action(row as isize, col as isize) {
                    valid_actions.push((row, col));
                }
            }
        }

        return valid_actions;
    }

    pub fn step(&mut self, row: usize, col: usize) -> bool {
        // Place the stone
        self.place(row, col);
        
        // Capture any stones
        self.capture(row, col);

        // Check if the game is over
        if self.is_done(row, col) {
            self.print();
            return true;
        } else {
            // Switch players
            self.current_player = -self.current_player;
            return false;
        }
    }

    pub fn print(&self) {
        for row in 0..self.size as usize {
            for col in 0..self.size as usize {
                print!("{} ", self.board[[row, col]]);
            }
            println!();
        }
    }
}


#[pymodule]
fn _lib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Pente>()?;
    Ok(())
}