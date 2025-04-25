


pub mod game_engine {
    use macroquad::prelude::*;

    use crate::physics_obj::physics_obj::PhysicsObj;

    #[derive(Default)]
    pub struct Game {
        ball: PhysicsObj,
    }

    impl Game {
        pub fn create() -> Game {
            let mut created_game = Game::default();

            created_game.ball = PhysicsObj {
                position: vec2(screen_width() / 2.0, screen_height() / 2.0),
                velocity: vec2(0.0, 0.0),
                radius: 30.0,
            };

            created_game
        }
        pub fn physics(&mut self) {
            self.ball.run_physics();
        }
        pub fn draw(&mut self) {
            draw_circle(self.ball.position.x, self.ball.position.y, self.ball.radius, RED);
        }
    }
}
