extern crate num;
#[macro_use]
extern crate num_derive;

use std::env;

mod game;
mod ship;
mod utils;

// TODO high score system

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 3 {
        panic!("Usage: cargo run <rounds> <distance>");
    }
    let rounds: u32;
    let mut distance: i32;
    match args.len() {
        3 => {
            rounds = utils::parse(&args[1]);
            distance = utils::parse(&args[2])
        }
        1 => {
            rounds = (rand::random::<u32>() % 20) + 10;
            distance = rand::random::<i32>() % 100;
            if distance < 0 {
                distance *= -1;
            }
        }
        _ => {
            panic!("Usage: cargo run [rounds] [distance]");
        }
    }
    let mut game = game::Game::new(rounds, distance);
    game.game_loop();
}
