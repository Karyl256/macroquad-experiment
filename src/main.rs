use macroquad::prelude::*;

mod game_engine;
mod physics_obj;
mod static_obj;

use macroquad::math::Vec2;

fn rotate_vec2(v: Vec2, angle: f32) -> Vec2 {
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();
    Vec2::new(
        v.x * cos_theta - v.y * sin_theta,
        v.x * sin_theta + v.y * cos_theta,
    )
}

fn window_config() -> Conf {
    Conf {
        window_width: 600,
        window_height: 700,
        window_resizable: false,
        window_title: String::from("Pinball"),
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut game = game_engine::game_engine::GameWorld::create();

    let mut timer = 0.0;
    loop {
        timer += get_frame_time();
        clear_background(Color::new(0.20, 0.3, 0.5, 1.0));
        if timer > 2.0 { game.physics(); }
        game.draw();
        next_frame().await;
    }
}
