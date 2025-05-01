use macroquad::prelude::*;

//Gravity strength in world space (positive Y is down)
pub const GRAVITY: Vec2 = Vec2::new(0.0, 500.0);
//Physics time scaling (multiplier)
pub const PHYSICS_SPEED: f32 = 1.0;
//Target fps for physics simulation steps, frame time is static dt
pub const PHYSICS_TARGET_FPS: f32 = 144.0;
pub const PHYSICS_TARGET_FRAMETIME: f32 = 1.0 / PHYSICS_TARGET_FPS * PHYSICS_SPEED;
//Cap to how many physics frames can happen in a game frame
pub const MAX_PHYSICS_UPDATES_PER_FRAME: u32 = 10;
//Flipper angular velocity
pub const FLIPPER_SPEED: f32 = 10.0;


pub mod game_engine {
    use std::f32::consts::PI;

    use macroquad::prelude::*;

    use crate::{physics_obj::physics_obj::PhysicsBody, static_obj::static_obj::StaticBody};
    use super::*;

    #[derive(Default)]
    pub struct GameWorld {
        ball: PhysicsBody,

        //colliders[0, 1] are (at lesat should be) flippers
        colliders: Vec<StaticBody>,
        physics_accumulated_time: f32,

        debug_draw_points: Vec<(Vec2, i32)>,
    }

    impl GameWorld {
        pub fn create() -> GameWorld {
            let mut created_game = GameWorld::default();

            created_game.ball = PhysicsBody::new(
                vec2(465.0, 600.0),
                vec2(0.0, -800.0),
                10.0,
            );

            created_game.initialize_world();

            created_game
        }

        pub fn physics(&mut self) {
            //Accumulated time
            self.physics_accumulated_time += get_frame_time() * PHYSICS_SPEED;

            let dt: f32 = PHYSICS_TARGET_FRAMETIME;

            let mut iteration = 0;
            while self.physics_accumulated_time > PHYSICS_TARGET_FRAMETIME && iteration < MAX_PHYSICS_UPDATES_PER_FRAME {
                self.physics_update(dt);
                
                iteration += 1;
                self.physics_accumulated_time -= PHYSICS_TARGET_FRAMETIME;
            }
        }

        pub fn physics_update(&mut self, dt: f32) {
            //Update flippers (index 0, 1)
            self.update_flipper(dt, 0, KeyCode::Left);
            self.update_flipper(dt, 1, KeyCode::Right);

            if is_key_pressed(KeyCode::Space) {
                self.ball = PhysicsBody::new(
                    vec2(465.0, 600.0),
                    vec2(0.0, -800.0),
                    self.ball.radius
                );
            }
            

            self.ball.update_physics(dt, &self.colliders, &mut self.debug_draw_points);
        }

        pub fn draw(&mut self) {
            draw_circle(self.ball.position.x, self.ball.position.y, self.ball.radius, RED);

            for obj in &self.colliders {
                obj.draw();
            }

            for point in &mut self.debug_draw_points {
                draw_circle(point.0.x, point.0.y, 5.0, YELLOW);
                point.1 -= 1;
            }
            self.debug_draw_points.retain(|p| p.1 > 0);
        }

        pub fn update_flipper(&mut self, dt: f32, index: usize, key: KeyCode) {
            let mut left_flipper: &mut StaticBody = self.colliders.get_mut(index).expect("No left flipper");
            if let StaticBody::Flipper {
                current_rotation, 
                rotation_max, 
                rotation_min, 
                angular_velocity,
                .. 
            } = &mut left_flipper {
                let up = (*rotation_min - *rotation_max).signum();
                if is_key_down(key) { *angular_velocity = -FLIPPER_SPEED * up; }
                else                { *angular_velocity =  FLIPPER_SPEED * up; }

                let previous_rotation = *current_rotation;
                *current_rotation = (*current_rotation + *angular_velocity * dt)
                    .clamp(
                        f32::min(*rotation_min, *rotation_max), 
                        f32::max(*rotation_min, *rotation_max)
                    );
                if previous_rotation == *current_rotation {
                    *angular_velocity = 0.0;
                }
            }
        }

        pub fn initialize_world(&mut self) {
            //FLIPPERS
            self.colliders.push(StaticBody::new_flipper(
                vec2(180.0, 660.0), vec2(30.0, 0.0), vec2(70.0, 10.0), 0.16 * PI, -0.5, PURPLE
            ));
            self.colliders.push(StaticBody::new_flipper(
                vec2(320.0, 660.0), vec2(-30.0, 0.0), vec2(70.0, 10.0), -0.16 * PI, 0.5, PURPLE
            ));


            //Floor
            self.colliders.push(StaticBody::new_rectangle(
                vec2(92.5, 612.0), vec2(201.0, 10.0), 0.16 * PI, GRAY
            ));
            self.colliders.push(StaticBody::new_rectangle(
                vec2(375.7, 629.91), vec2(127.2, 10.0), -0.16 * PI, GRAY
            ));
            //Walls
            self.colliders.push(StaticBody::new_rectangle(
                vec2(490.0, 350.0), vec2(20.0, 700.0), 0.0, GRAY
            ));
            self.colliders.push(StaticBody::new_rectangle(
                vec2(10.0, 350.0), vec2(20.0, 700.0), 0.0, GRAY
            ));
            //Roof
            self.colliders.push(StaticBody::new_rectangle(
                vec2(250.0, 10.0), vec2(500.0, 20.0), 0.0, GRAY
            ));
            //Inside wall
            self.colliders.push(StaticBody::new_rectangle(
                vec2(439.0, 450.0),  vec2(20.0, 420.0), 0.0, GREEN
            ));

            //Enter curves: outside, inside
            self.colliders.push(StaticBody::new_curve(
                vec2(250.0, 250.0), 230.0, 20.0, -2.12, 0.0, 30, BLUE
            ));
            self.colliders.push(StaticBody::new_curve(
                vec2(250.0, 250.0), 199.0, -20.0, -0.7, 0.0, 30, BLUE
            ));

            self.colliders.push(StaticBody::new_curve(
                vec2(75.0, 75.0), 50.0, 10.0, PI * 0.65, PI * -0.12, 100, PINK
            ));

            self.colliders.push(StaticBody::new_circle(vec2(75.0, 75.0), 12.5, PINK));

            //Outside continue
            self.colliders.push(StaticBody::new_curve(
                vec2(250.0, 250.0), 230.0, 20.0, PI * 0.75, -2.55, 30, BLUE
            ));
            
        }
    }
}
