extern crate num;
#[macro_use]
extern crate num_derive;

use std::env;

mod game;
mod ship;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: cargo run <rounds> <distance>");
    }
    let rounds: u32 = match args[1].parse::<u32>() {
        Ok(num) => num,
        Err(_) => panic!("<rounds> needs to be a number."),
    };
    let distance: i32 = match args[2].parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!("<distance> needs to be a number."),
    };
    let mut game = game::Game::new(rounds, distance); //TODO prettier declaration?
    game.game_loop();
}
