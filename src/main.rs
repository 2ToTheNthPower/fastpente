mod board;
mod player;
mod game;
use game::Game;
use std::io;

fn main() {
    println!("Welcome to Cargo Pente!");
    let size: usize = 19; //input.trim().parse().expect("Please type a number!");
    let num_players: usize = 2;

    // Run 10000 games
    let mut num_games = 0;
    let mut num_draws = 0;
    let mut num_wins = vec![0; num_players];
    while num_games < 10000 {
        let mut game = Game::new(size, num_players);
        let outcome = game.run();
        if outcome.is_draw{
            num_draws += 1;
        } else {
            num_wins[outcome.winner.unwrap()] += 1;
        }
        num_games += 1;
    }
}



