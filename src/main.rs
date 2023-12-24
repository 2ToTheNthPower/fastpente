// Path: src/main.rs
// Compare this snippet from src/main.rs:
use game::Game;
use std::io;
mod board;
mod player;
mod game;

fn main() {
    println!("Welcome to Cargo Go!");
    println!("Please enter the board size:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let size: usize = input.trim().parse().expect("Please type a number!");
    println!("Please enter the number of players:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_players: usize = input.trim().parse().expect("Please type a number!");
    let mut game = Game::new(size, num_players);
    game.run();
}



