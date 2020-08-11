extern crate num;
#[macro_use]
extern crate num_derive;

use std::env;

mod game;
mod ship;
mod utils;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: cargo run <rounds> <distance>");
    }
    let rounds: u32 = utils::parse(&args[1]);
    let distance: i32 = utils::parse(&args[2]);
    let mut game = game::Game::new(rounds, distance); //TODO prettier declaration?
    game.game_loop();
}
