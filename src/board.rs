use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Piece {
    Empty,
    Black,
    White,
    Red,
    Green,  // You can add more colors or types of pieces if you like
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {  // Corrected by removing the dereference
            Piece::Empty => write!(f, "."),
            Piece::Black => write!(f, "B"),
            Piece::White => write!(f, "W"),
            Piece::Red   => write!(f, "R"),
            Piece::Green => write!(f, "G"),
        }
    }
}

pub struct Board {
    pub size: usize,
    pub grid: Vec<Vec<Piece>>,
}

impl Board {
    // Initialize a new board of given size
    // Size should default to 19x19 if not specified
    pub fn new(size:usize) -> Board {
        Board {
            size,
            grid: vec![vec![Piece::Empty; size]; size],
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        for row in self.grid.iter(){
            for cell in row.iter() {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
