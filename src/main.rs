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

    // Run 1 game
    let mut num_games = 0;
    let mut num_draws = 0;
    let mut num_wins = vec![0; num_players];
    while num_games < 100000 {
        num_games += 1;
        let mut game = Game::new(size, num_players);
        let (board, reward, done, outcome) = game.run(false, );

        let file_path = format!("games/game_{}.bin", num_games);
        // game.save(&file_path);
        // let loaded_game = Game::load(&file_path);
        println!("Game {} finished", num_games);
    }
    let duration = start.elapsed();
    println!("Average time per game: {:?}", duration / num_games);
}

// use ndarray::array;

// fn main() {
//     let a1 = array![1, 2, 3, 4];

//     let a2 = array![[1, 2],
//                     [3, 4]];

//     let a3 = array![[[1, 2], [3, 4]],
//                     [[5, 6], [7, 8]]];

//     println!("Item at position (1, 0) in a2: {}", a2[[1, 0]]);
// }




