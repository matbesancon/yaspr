extern crate snake_game;

use snake_game::Game;
use std::string::ToString;

fn main() {
    let g = Game::new(100, 100);
    let _s = g.to_string();

    println!("Hello, world!");
}
