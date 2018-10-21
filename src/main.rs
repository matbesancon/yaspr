extern crate snake_game;

use snake_game::Game;
use snake_game::game_loop::App;

fn main() {
    let mut a = App::new(20, 20);
    let score = a.event_loop();
    println!("Your score: {}", score);
}
