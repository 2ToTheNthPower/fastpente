use ndarray::Array2;
use std::fmt;

mod lib;

use lib::Pente;

fn main() {
    let mut game = Pente::new();
    let mut done = false;

    while !done {
        println!("Player {}'s turn", game.current_player);
        println!("Enter row: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let row = input.trim().parse::<usize>().unwrap();
        println!("Enter col: ");
        input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let col = input.trim().parse::<usize>().unwrap();
        
        done = game.step(row, col);
        game.print();
    }
}