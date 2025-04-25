use macroquad::prelude::*;

mod game_engine;
mod physics_obj;

#[macroquad::main("Program")]
async fn main() {
    let mut game = game_engine::game_engine::Game::create();
    loop {
        clear_background(Color::new(0.20, 0.3, 0.5, 1.0));
        game.physics();
        game.draw();
        next_frame().await;
    }
}
