extern crate num;
#[macro_use]
extern crate num_derive;

use std::env;

mod game;
mod ship;

fn main() {
    //TODO check for best practices, do better error checking etc
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: cargo run <rounds> <distance>");
    }
    let rounds = args[1].parse::<u32>().unwrap();
    let distance = args[2].parse::<i32>().unwrap();
    let mut game = game::Game::new(rounds, distance); //TODO prettier declaration?
    game.game_loop();
}

