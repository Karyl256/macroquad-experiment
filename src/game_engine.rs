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
pub const FLIPPER_SPEED: f32 = 7.5;
//Launcher: maximum charge time length, maximum force
pub const LAUNCHER_MAX_TIME: f32 = 2.0;
pub const LAUNCHER_MAX_STRENGTH: f32 = 820.0;


pub mod game_engine {
    use std::f32::consts::PI;

    use macroquad::prelude::*;

    use crate::{circle, curve, flipper, physics_obj::physics_obj::PhysicsBody, rect, static_obj::static_obj::StaticBody};
    use super::*;

    #[derive(Default)]
    pub struct GameWorld {
        ball: PhysicsBody,

        //colliders[0, 1] are (at lesat should be) flippers
        colliders: Vec<StaticBody>,
        physics_accumulated_time: f32,

        launcher_accumulator: f32,

        debug_draw_points: Vec<(Vec2, i32)>,
    }

    impl GameWorld {
        pub fn create() -> GameWorld {
            let mut created_game = GameWorld::default();

            created_game.ball = PhysicsBody::new(
                vec2(465.0, 600.0),
                vec2(0.0, 0.0),
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

            //If R pressed or ball is out of bounds, restart
            //TODO: ADD HEALTH SYSTEM
            if is_key_pressed(KeyCode::R) || self.ball.position.y > (screen_height() + 300.0) {
                self.ball = PhysicsBody::new(
                    vec2(465.0, 600.0),
                    vec2(0.0, 0.0),
                    self.ball.radius
                );
            }

            if is_key_down(KeyCode::Space) {
                //Add dt to launcher accumulator, while giving it a limit
                self.launcher_accumulator = (self.launcher_accumulator + dt).clamp(0.0, LAUNCHER_MAX_TIME);
            }
            if is_key_released(KeyCode::Space) 
            {
                if (460.0 < self.ball.position.x && self.ball.position.x < 470.0) && (595.0 < self.ball.position.y && self.ball.position.y < 605.0) {
                    self.ball.velocity.y = -(self.launcher_accumulator/LAUNCHER_MAX_TIME) * LAUNCHER_MAX_STRENGTH;
                }
                self.launcher_accumulator = 0.0;
            }
            

            self.ball.update_physics(dt, &self.colliders, &mut self.debug_draw_points);
        }

        pub fn draw(&mut self) {
            draw_circle(self.ball.position.x, self.ball.position.y, self.ball.radius, Color::from_rgba(190, 190, 200, 255));

            //Render map
            for obj in &self.colliders {
                obj.draw();
            }

            //Render launcher
            let launcher_percentage = self.launcher_accumulator / LAUNCHER_MAX_TIME;
            draw_rectangle(460.0, 625.0, 10.0, (-1.0 + launcher_percentage * 0.9) * 15.0, YELLOW);

            //Draw debug points
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
            // Flippers
            flipper!(self, vec2(180.0, 630.0), vec2( 24.0, 0.0), vec2(60.0, 10.0),  0.16 * PI, -0.5, PURPLE);
            flipper!(self, vec2(320.0, 630.0), vec2(-24.0, 0.0), vec2(60.0, 10.0), -0.16 * PI,  0.5, PURPLE);

            // Floor
            rect!(self, vec2(116.0,  595.1),  vec2(144.8, 10.0),  0.16 * PI, GRAY);
            rect!(self, vec2(375.7, 599.91), vec2(127.2, 10.0), -0.16 * PI, GRAY);

            // Lower floor
            rect!(self, vec2(87.69, 619.2), vec2(77.3, 10.0), 0.16 * PI, GRAY);
            rect!(self, vec2(375.7, 639.91), vec2(127.2, 10.0), -0.16 * PI, GRAY);

            // Walls
            rect!(self, vec2(490.0, 350.0), vec2(20.0, 700.0), 0.0, GRAY);
            rect!(self, vec2(10.0, 350.0), vec2(20.0, 700.0), 0.0, GRAY);

            // Roof
            rect!(self, vec2(250.0, 10.0), vec2(500.0, 20.0), 0.0, GRAY);

            // Inside wall
            rect!(self, vec2(445.0, 435.0), vec2(10.0, 390.0), 0.0, GRAY);
            rect!(self, vec2(465.0, 620.0), vec2(30.0, 20.0), 0.0, GRAY);

            //Opposite inside wall
            rect!(self, vec2(55.0, 510.0), vec2(10.0, 110.0), 0.0, GRAY);

            // Enter curves
            curve!(self, vec2(250.0, 250.0), 230.0, 20.0, -2.12, 0.0, 60, GRAY);
            curve!(self, vec2(250.0, 250.0), 200.0, -10.0, -1.1, 0.0, 0, GRAY);
            curve!(self, vec2(75.0, 75.0), 50.0, 10.0, PI * 0.65, PI * -0.11, 100, PINK);

            // Bumper
            circle!(self, vec2(75.0, 75.0), 12.5, PINK);

            // Outside continue
            curve!(self, vec2(250.0, 250.0), 230.0, 10.0, PI * 0.75, -2.55, 30, GRAY);

            // Tunnel
            curve!(self, vec2(250.0, 250.0), 190.0, 10.0, -1.15, 0.5, 40, GRAY);
            curve!(self, vec2(250.0, 250.0), 165.0, -10.0, -1.15, 0.3, 40, GRAY);
            curve!(self, vec2(250.0, 250.0), 155.0, 10.0, -1.15, 0.3, 0, GREEN);

            // Top 2 splitters
            rect!(self, vec2(230.0, 130.0), vec2(10.0, 30.0), 0.0, GRAY);
            rect!(self, vec2(270.0, 130.0), vec2(10.0, 30.0), 0.0, GRAY);

            //Left top abomination
            curve!(self, vec2(250.0, 250.0), 200.0, -5.0, -2.24, -1.96, 20, GRAY);
            rect!(self, vec2(130.0, 110.0), vec2(10.0, 30.0), 0.0, GRAY);
            rect!(self, vec2(170.0, 85.0), vec2(10.0, 30.0), 0.0, GRAY);
            rect!(self, vec2(150.0, 112.0), vec2(54.0, 10.0), PI * -0.175, GRAY);
            rect!(self, vec2(150.0, 95.0), vec2(45.0, 25.0), PI * -0.175, GRAY);
            
        }
    }



}

