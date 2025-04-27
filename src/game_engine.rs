use macroquad::prelude::*;

pub const GRAVITY: Vec2 = Vec2::new(0.0, 1000.0);
//How fast games physics run
pub const PHYSICS_SPEED: f32 = 1.0;
//What is expected amount of fps
pub const PHYSICS_TARGET_FPS: f32 = 144.0;
pub const PHYSICS_TARGET_FRAMETIME: f32 = 1.0 / PHYSICS_TARGET_FPS * PHYSICS_SPEED;
//Maximum of how many frames can you try to gain back to be on time in a frame
pub const MAX_PHYSICS_UPDATES_PER_FRAME: u32 = 10;



pub mod game_engine {
    use macroquad::prelude::*;

    use crate::{physics_obj::physics_obj::PhysicsObj, static_obj::static_obj::StaticObj};
    use super::*;

    #[derive(Default)]
    pub struct Game {
        ball: PhysicsObj,

        physics_objects: Vec<StaticObj>,
        physics_buffered_time: f32,

        debug_queue: Vec<Vec2>,
    }

    impl Game {
        pub fn create() -> Game {
            let mut created_game = Game::default();

            created_game.ball = PhysicsObj::new(
                vec2(550.0, 500.0),
                vec2(0.0, -1000.0),
                15.0,
            );

            created_game.create_map();

            created_game
        }

        pub fn physics(&mut self) {
            //Buffered time
            self.physics_buffered_time += get_frame_time() * PHYSICS_SPEED;

            let dt: f32 = PHYSICS_TARGET_FRAMETIME;

            let iteration = 0;
            while self.physics_buffered_time > PHYSICS_TARGET_FRAMETIME && iteration < MAX_PHYSICS_UPDATES_PER_FRAME {
                self.ball.run_physics(dt, &self.physics_objects, &mut self.debug_queue);
                

                self.physics_buffered_time -= PHYSICS_TARGET_FRAMETIME;
            }
        }

        pub fn draw(&mut self) {
            draw_circle(self.ball.position.x, self.ball.position.y, self.ball.radius, RED);

            //Draw static objects
            for obj in &self.physics_objects {
                obj.draw();
            }

            for point in &self.debug_queue {
                draw_circle(point.x, point.y, 5.0, YELLOW)
            }
            self.debug_queue.clear();
        }

        pub fn create_map(&mut self) {

            //Floor
            self.physics_objects.push(StaticObj::new_rectangle(
                vec2(100.0, 580.0), vec2(200.0, 40.0), 0.0, GRAY
            ));
            self.physics_objects.push(StaticObj::new_rectangle(
                vec2(500.0, 580.0), vec2(200.0, 40.0), 0.0, GRAY
            ));
            //Walls
            self.physics_objects.push(StaticObj::new_rectangle(
                vec2(590.0, 300.0), vec2(20.0, 600.0), 0.0, GRAY
            ));
            self.physics_objects.push(StaticObj::new_rectangle(
                vec2(10.0, 300.0), vec2(20.0, 600.0), 0.0, GRAY
            ));
            //Roof
            self.physics_objects.push(StaticObj::new_rectangle(
                vec2(300.0, 10.0), vec2(600.0, 20.0), 0.0, GRAY
            ));
            //Angled
            self.physics_objects.push(StaticObj::new_rectangle(
                vec2(550.0, 50.0), vec2(60.0, 10.0), 0.9, PURPLE
            ));
            self.physics_objects.push(StaticObj::new_rectangle(
                vec2(300.0, 300.0), vec2(60.0, 10.0), 0.7, PURPLE
            ));
        }
    }
}
