extern crate num;
#[macro_use]
extern crate num_derive;

mod game;
mod ship;

fn main() {
    let base_rounds = 20; // TODO change
    let base_distance = 100; // TODO change
    let mut game = game::Game::new(base_rounds, base_distance); //TODO prettier declaration?
    game.game_loop();
}

