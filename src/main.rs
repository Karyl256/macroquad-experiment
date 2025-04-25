use macroquad::{shapes::draw_circle, window::{clear_background, screen_height, screen_width}};
use macroquad::prelude::*;

const GRAVITY: Vec2 = Vec2::new(0.0, 1000.0);

struct PhysicsObj {
    position: Vec2,
    velocity: Vec2,
}

impl PhysicsObj {

    #[allow(dead_code)]
    fn empty() -> PhysicsObj {
        PhysicsObj {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0) 
        }
    }

    #[allow(dead_code)]
    fn new(position: Vec2, velocity: Vec2) -> PhysicsObj {
        PhysicsObj {
            position,
            velocity,
        }
    }

    #[allow(dead_code)]
    fn phys(&mut self) {
        self.velocity += GRAVITY * get_frame_time();
        self.position += self.velocity * get_frame_time();
    }
}

#[macroquad::main("Program")]
async fn main() {
    let mut ball = PhysicsObj::new(
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        vec2(0.0, 0.0)
    );

    loop {
        clear_background(Color::new(0.20, 0.3, 0.5, 1.0));

        ball.phys();

        draw_circle(ball.position.x, ball.position.y, 50.0, RED);
        
        next_frame().await;
    }
}

