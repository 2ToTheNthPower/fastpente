mod board;
mod random_player;
mod mcts_player;
mod game;
use game::Game;
use std::io;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    println!("Welcome to Cargo Pente!");
    let size: usize = 19; //input.trim().parse().expect("Please type a number!");
    let num_players: usize = 2;

    // Run 10000 games
    let mut num_games = 0;
    let mut num_draws = 0;
    let mut num_wins = vec![0; num_players];
    while num_games < 10000 {
        num_games += 1;
        let mut game = Game::new(size, num_players);
        let (board, reward, done, outcome) = game.run(false);

        let file_path = format!("games/game_{}.bin", num_games);
        // game.save(&file_path);
        // let loaded_game = Game::load(&file_path);
    }
    let duration = start.elapsed();
    println!("Average time per game: {:?}", duration / num_games);
}



