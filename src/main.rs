use macroquad::prelude::*;

mod game_engine;
mod physics_obj;
mod static_obj;
pub mod helper;

fn window_config() -> Conf {
    Conf {
        window_width: 650,
        window_height: 700,
        window_resizable: false,
        window_title: String::from("Pinball"),
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut game = game_engine::game_engine::GameWorld::create().await;

    loop {
        clear_background(Color::new(0.20, 0.3, 0.5, 1.0));
        game.physics();
        game.draw();
        next_frame().await;
    }
}
