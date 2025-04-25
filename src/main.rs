use macroquad::{shapes::draw_circle, window::{clear_background, screen_height, screen_width}};
use macroquad::prelude::*;

const GRAVITY: Vec2 = Vec2::new(0.0, 1000.0);
const SUBSTEP: u32 = 10;
const SPEED: u32 = 1;

struct PhysicsObj {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
}

impl PhysicsObj {

    #[allow(dead_code)]
    fn empty() -> PhysicsObj {
        PhysicsObj {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            radius: 1.0,
        }
    }

    #[allow(dead_code)]
    fn new(position: Vec2, velocity: Vec2, radius: f32) -> PhysicsObj {
        PhysicsObj {
            position,
            velocity,
            radius,
        }
    }

    #[allow(dead_code)]
    fn run_physics(&mut self) {
        let dt: f32 = 0.01 / SUBSTEP as f32;

        #[allow(unused_mut)]
        let mut acceleration = GRAVITY;

        for _n in 0..(SPEED * SUBSTEP) {
            if self.position.y + self.radius > screen_height() {
                self.velocity.y = -self.velocity.y;
                self.position.y = screen_height() - self.radius;
            }

            self.velocity += acceleration * dt;
            self.position += self.velocity * dt;
        }
    }
}

#[macroquad::main("Program")]
async fn main() {
    let mut ball = PhysicsObj::new(
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        vec2(0.0, 0.0),
        30.0
    );

    loop {
        clear_background(Color::new(0.20, 0.3, 0.5, 1.0));

        ball.run_physics();
        println!("{}", get_fps());

        draw_circle(ball.position.x, ball.position.y, ball.radius, RED);
        
        next_frame().await;
    }
}

