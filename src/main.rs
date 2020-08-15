extern crate num;
#[macro_use]
extern crate num_derive;

use std::env;

mod game;
mod ship;
mod utils;

fn main() {
    let args: Vec<_> = env::args().collect();
    let rounds: u32;
    let distance: i32;
    match args.len() {
        3 => {
            rounds = utils::parse(&args[1]);
            distance = utils::parse(&args[2])
        }
        1 => {
            rounds = (rand::random::<u32>() % 15) + 10;
            distance = (rand::random::<u32>() % 100) as i32;
        }
        _ => {
            panic!("Usage: cargo run [rounds] [distance]");
        }
    }
    let mut game = game::Game::new(rounds, distance);
    game.game_loop();
}
