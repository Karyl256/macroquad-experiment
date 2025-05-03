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

    use crate::{circle, curve, flipper, physics_obj::physics_obj::PhysicsBody, rect, static_obj::static_obj::StaticBody, helper::helper::format_number};
    use super::*;

    #[derive(Default)]
    pub struct GameWorld {
        ball: PhysicsBody,

        //colliders[0, 1] are (at least should be) flippers
        colliders: Vec<StaticBody>,
        physics_accumulated_time: f32,

        launcher_accumulator: f32,

        font: Option<Font>,
        debug_draw_points: Vec<(Vec2, i32)>,
        score: f32,
        lives: u32,
    }

    impl GameWorld {
        pub async fn create() -> GameWorld {
            let mut created_game = GameWorld::default();

            //Load font
            created_game.font = Some(load_ttf_font("sans-medium.ttf").await.expect("No file"));

            //Create ball
            created_game.ball = PhysicsBody::new(vec2(465.0, 600.0), vec2(0.0, 0.0),10.0);
            created_game.score = 0.0;
            created_game.lives = 3;

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

            //Find all spinners
            for s in self.colliders.iter_mut().filter(|e| matches!(e, StaticBody::Spinner { .. })) {
                if let StaticBody::Spinner { acc_velocity, top_down_rotation, .. } = s {
                    *top_down_rotation += *acc_velocity * dt;
                    *acc_velocity = acc_velocity.signum() * f32::max(acc_velocity.abs() - dt * 5.0, 0.0);
                    self.score += acc_velocity.abs() * dt * 100.0;
                }
            }

            //If R pressed or ball is out of bounds, restart
            if is_key_pressed(KeyCode::R) || self.ball.position.y > (screen_height() + 300.0) {
                if self.lives > 0 {
                    self.lives -= 1;

                    self.restart_ball();
                }
            }
            if is_key_pressed(KeyCode::E) {
                //Reset all spinners velociy
                for s in self.colliders.iter_mut().filter(|e| matches!(e, StaticBody::Spinner { .. })) {
                    if let StaticBody::Spinner { acc_velocity, .. } = s {
                        *acc_velocity = 0.0;
                    }
                }

                self.lives = 3;
                self.score = 0.0;
                self.restart_ball();
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
            
            self.ball.update_physics(dt, &mut self.colliders, &mut self.score, &mut self.debug_draw_points);
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

            //Render score and lives
            self.draw_number(format_number((self.score / 100.0) as i32 * 100), vec2(628.0, 25.0), 12.0, 25.0, 2.0);
            self.draw_number(self.lives.to_string(), vec2(624.0, 100.0), 15.0, 30.0, 3.0);

            //Draw debug points
            for point in &mut self.debug_draw_points {
                //draw_circle(point.0.x, point.0.y, 5.0, YELLOW);
                point.1 -= 1;
            }

            self.debug_draw_points.retain(|p| p.1 > 0);
        }

        pub fn draw_number(&mut self, num: String, corner: Vec2, width: f32, height: f32, thickness: f32) {
            let mut current_corner = corner;
        
            for c in num.chars().rev() {
                match c {
                    '0' => {
                        draw_rectangle(current_corner.x, current_corner.y, width, thickness, WHITE); // top
                        draw_rectangle(current_corner.x, current_corner.y + height - thickness, width, thickness, WHITE); // bottom
                        draw_rectangle(current_corner.x, current_corner.y, thickness, height, WHITE); // left
                        draw_rectangle(current_corner.x + width - thickness, current_corner.y, thickness, height, WHITE); // right
                    }
                    '1' => {
                        draw_rectangle(current_corner.x + width - thickness, current_corner.y, thickness, height, WHITE);
                    }
                    '2' => {
                        draw_rectangle(current_corner.x, current_corner.y, width, thickness, WHITE); // top
                        draw_rectangle(current_corner.x, current_corner.y + height / 2.0 - thickness / 2.0, width, thickness, WHITE); // middle
                        draw_rectangle(current_corner.x, current_corner.y + height - thickness, width, thickness, WHITE); // bottom
                        draw_rectangle(current_corner.x + width - thickness, current_corner.y, thickness, height / 2.0, WHITE); // top right
                        draw_rectangle(current_corner.x, current_corner.y + height / 2.0, thickness, height / 2.0, WHITE); // bottom left
                    }
                    '3' => {
                        draw_rectangle(current_corner.x, current_corner.y, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y + height / 2.0 - thickness / 2.0, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y + height - thickness, width, thickness, WHITE);
                        draw_rectangle(current_corner.x + width - thickness, current_corner.y, thickness, height, WHITE);
                    }
                    '4' => {
                        draw_rectangle(current_corner.x + width - thickness, current_corner.y, thickness, height, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y, thickness, height / 2.0 + thickness / 2.0, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y + height / 2.0 - thickness / 2.0, width, thickness, WHITE);
                    }
                    '5' => {
                        draw_rectangle(current_corner.x, current_corner.y, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y + height / 2.0 - thickness / 2.0, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y + height - thickness, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y, thickness, height / 2.0, WHITE);
                        draw_rectangle(current_corner.x + width - thickness, current_corner.y + height / 2.0, thickness, height / 2.0, WHITE);
                    }
                    '6' => {
                        draw_rectangle(current_corner.x, current_corner.y, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y + height / 2.0 - thickness / 2.0, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y + height - thickness, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y, thickness, height, WHITE);
                        draw_rectangle(current_corner.x + width - thickness, current_corner.y + height / 2.0, thickness, height / 2.0, WHITE);
                    }
                    '7' => {
                        draw_rectangle(current_corner.x, current_corner.y, width, thickness, WHITE);
                        draw_rectangle(current_corner.x + width - thickness, current_corner.y, thickness, height, WHITE);
                    }
                    '8' => {
                        draw_rectangle(current_corner.x, current_corner.y, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y + height / 2.0 - thickness / 2.0, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y + height - thickness, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y, thickness, height, WHITE);
                        draw_rectangle(current_corner.x + width - thickness, current_corner.y, thickness, height, WHITE);
                    }
                    '9' => {
                        draw_rectangle(current_corner.x, current_corner.y, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y + height / 2.0 - thickness / 2.0, width, thickness, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y + height - thickness, width, thickness, WHITE);
                        draw_rectangle(current_corner.x + width - thickness, current_corner.y, thickness, height, WHITE);
                        draw_rectangle(current_corner.x, current_corner.y, thickness, height / 2.0, WHITE);
                    }
                    ' ' => {
                        current_corner += vec2((width + thickness + 3.0) / 1.5, 0.0);
                    }
                    _ => {}
                }
        
                current_corner -= vec2(width + thickness + 3.0, 0.0); // spacing between digits
            }
        }
        
        pub fn restart_ball(&mut self) {
            self.ball = PhysicsBody::new(
                vec2(465.0, 600.0),
                vec2(0.0, 0.0),
                self.ball.radius
            );
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
            rect!(self, vec2(116.0, 595.1),  vec2(144.8, 10.0),  0.16 * PI, GRAY);
            rect!(self, vec2(384.0, 595.1), vec2(144.8, 10.0), -0.16 * PI, GRAY);

            // Lower floor
            rect!(self, vec2(100.0, 627.0), vec2(109.0, 10.0), 0.16 * PI, GRAY);
            rect!(self, vec2(400.0, 627.0), vec2(109.0, 10.0), -0.16 * PI, GRAY);

            // Walls
            rect!(self, vec2(490.0, 350.0), vec2(20.0, 700.0), 0.0, GRAY);
            rect!(self, vec2(10.0, 350.0), vec2(20.0, 700.0), 0.0, GRAY);

            // Roof
            rect!(self, vec2(250.0, 10.0), vec2(500.0, 20.0), 0.0, GRAY);

            // Inside wall
            rect!(self, vec2(445.0, 435.0), vec2(10.0, 390.0), 0.0, GRAY);
            rect!(self, vec2(465.0, 620.0), vec2(30.0, 20.0), 0.0, GRAY);

            //Opposite inside wall
            rect!(self, vec2(55.0, 530.0), vec2(10.0, 70.0), 0.0, GRAY);
            rect!(self, vec2(55.0, 630.0), vec2(10.0, 50.0), 0.0, GRAY);
            rect!(self, vec2(35.0, 650.0), vec2(30.0, 10.0), 0.0, LIGHTGRAY, 200.0);
            rect!(self, vec2(50.0, 450.0), vec2(100.0, 10.0), PI * -0.25, GRAY);

            // Enter curves
            curve!(self, vec2(250.0, 250.0), 230.0, 20.0, -2.11, 0.0, 30, GRAY);
            curve!(self, vec2(250.0, 250.0), 200.0, -10.0, -1.15, 0.0, 0, GRAY);
            curve!(self, vec2(75.0, 75.0), 50.0, 50.0, PI * 0.665, PI * -0.13, 30, GRAY);

            // Bumper
            circle!(self, vec2(75.0, 75.0), 15.0, WHITE, 100.0);

            // Outside continue
            curve!(self, vec2(250.0, 250.0), 230.0, 110.0, PI * 0.75, -2.55, 30, GRAY);

            // Tunnel
            curve!(self, vec2(250.0, 250.0), 190.0, 10.0, -1.15, 0.5, 30, GRAY);
            curve!(self, vec2(250.0, 250.0), 165.0, -10.0, -1.15, 0.3, 30, GRAY);
            curve!(self, vec2(250.0, 250.0), 155.0, 10.0, -1.15, 0.3, 0, GREEN);

            // Top 2 splitters
            rect!(self, vec2(230.0, 130.0), vec2(10.0, 30.0), 0.0, GRAY);
            rect!(self, vec2(270.0, 130.0), vec2(10.0, 30.0), 0.0, GRAY);

            //Middle angled
            rect!(self, vec2(245.0, 310.0), vec2(40.0, 10.0), PI * 0.16, YELLOW, 50.0);
            rect!(self, vec2(250.0, 300.0), vec2(50.0, 20.0), PI * 0.16, GRAY);

            //Middle bumpers
            circle!(self, vec2(260.0, 280.0), 15.0, WHITE, 100.0);
            circle!(self, vec2(310.0, 220.0), 15.0, WHITE, 100.0);
            circle!(self, vec2(210.0, 230.0), 15.0, WHITE, 100.0);

            //Left top abomination
            curve!(self, vec2(250.0, 250.0), 200.0, -5.0, -2.24, -1.96, 20, GRAY);
            rect!(self, vec2(130.0, 110.0), vec2(10.0, 30.0), 0.0, GRAY);
            rect!(self, vec2(170.0, 85.0), vec2(10.0, 30.0), 0.0, GRAY);
            rect!(self, vec2(150.0, 112.0), vec2(54.0, 10.0), PI * -0.175, YELLOW, 50.0);
            rect!(self, vec2(150.0, 94.0), vec2(45.0, 21.0), PI * -0.175, GRAY);

            //Bottom left bumper
            rect!(self, vec2(130.0, 535.0), vec2(25.0, 40.0), 0.0, DARKBLUE, 0.0);
            rect!(self, vec2(124.0, 505.0), vec2(13.0, 25.0), 0.0, DARKBLUE, 0.0);
            rect!(self, vec2(136.0, 545.0), vec2(37.0, 20.0), 0.0, DARKBLUE, 0.0);
            rect!(self, vec2(140.0, 520.0), vec2(8.0, 70.0), PI * -0.16, WHITE, 150.0);
            rect!(self, vec2(139.0, 522.0), vec2(5.0, 75.0), PI * -0.16, DARKBLUE, 0.0);

            //Bottom right bumper
            rect!(self, vec2(370.0, 535.0), vec2(25.0, 40.0), 0.0, DARKBLUE, 0.0);
            rect!(self, vec2(376.0, 505.0), vec2(13.0, 25.0), 0.0, DARKBLUE, 0.0);
            rect!(self, vec2(364.0, 545.0), vec2(37.0, 20.0), 0.0, DARKBLUE, 0.0);
            rect!(self, vec2(360.0, 520.0), vec2(8.0, 70.0), PI * 0.16, WHITE, 150.0);
            rect!(self, vec2(361.0, 522.0), vec2(5.0, 75.0), PI * 0.16, DARKBLUE, 0.0);
            
            //Spinner in tunnel
            self.colliders.push(StaticBody::Spinner { 
                position: vec2(425.0, 280.0), dimensions: vec2(20.0, 20.0), rotation: 0.2, acc_velocity: 0.0, top_down_rotation: 0.0, color: LIGHTGRAY 
            });
        }
    }



}

