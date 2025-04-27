use macroquad::prelude::*;

//Gravity strength in world space (positive Y is down)
pub const GRAVITY: Vec2 = Vec2::new(0.0, 1000.0);
//Physics time scaling (multiplier)
pub const PHYSICS_SPEED: f32 = 1.0;
//Target fps for physics simulation steps, frame time is static dt
pub const PHYSICS_TARGET_FPS: f32 = 144.0;
pub const PHYSICS_TARGET_FRAMETIME: f32 = 1.0 / PHYSICS_TARGET_FPS * PHYSICS_SPEED;
//Cap to how many physics frames can happen in a game frame
pub const MAX_PHYSICS_UPDATES_PER_FRAME: u32 = 10;



pub mod game_engine {
    use macroquad::prelude::*;

    use crate::{physics_obj::physics_obj::PhysicsBody, static_obj::static_obj::StaticBody};
    use super::*;

    #[derive(Default)]
    pub struct GameWorld {
        ball: PhysicsBody,

        colliders: Vec<StaticBody>,
        physics_accumulated_time: f32,

        debug_draw_points: Vec<Vec2>,
    }

    impl GameWorld {
        pub fn create() -> GameWorld {
            let mut created_game = GameWorld::default();

            created_game.ball = PhysicsBody::new(
                vec2(565.0, 500.0),
                vec2(0.0, -2000.0),
                15.0,
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
                self.ball.update_physics(dt, &self.colliders, &mut self.debug_draw_points);
                
                iteration += 1;
                self.physics_accumulated_time -= PHYSICS_TARGET_FRAMETIME;
            }
        }

        pub fn draw(&mut self) {
            draw_circle(self.ball.position.x, self.ball.position.y, self.ball.radius, RED);

            for obj in &self.colliders {
                obj.draw();
            }

            for point in &self.debug_draw_points {
                draw_circle(point.x, point.y, 5.0, YELLOW)
            }
            self.debug_draw_points.clear();
        }

        pub fn initialize_world(&mut self) {

            //Floor
            self.colliders.push(StaticBody::new_rectangle(
                vec2(100.0, 580.0), vec2(200.0, 40.0), 0.0, GRAY
            ));
            self.colliders.push(StaticBody::new_rectangle(
                vec2(500.0, 580.0), vec2(200.0, 40.0), 0.0, GRAY
            ));
            //Walls
            self.colliders.push(StaticBody::new_rectangle(
                vec2(590.0, 300.0), vec2(20.0, 600.0), 0.0, GRAY
            ));
            self.colliders.push(StaticBody::new_rectangle(
                vec2(10.0, 300.0), vec2(20.0, 600.0), 0.0, GRAY
            ));
            //Roof
            self.colliders.push(StaticBody::new_rectangle(
                vec2(300.0, 10.0), vec2(600.0, 20.0), 0.0, GRAY
            ));
            //Inside wall
            self.colliders.push(StaticBody::new_rectangle(
                vec2(539.0, 400.0),  vec2(20.0, 320.0), 0.0, GREEN
            ));
            //Angled
            self.colliders.push(StaticBody::new_rectangle(
                vec2(300.0, 300.0), vec2(60.0, 10.0), 0.7, PURPLE
            ));
            self.create_curve(vec2(350.0, 250.0), 230.1, 20.0, -std::f32::consts::FRAC_PI_2, 0.0, 20, BLUE);
            self.create_curve(vec2(350.0, 260.0), 180.0, 20.0, -std::f32::consts::FRAC_PI_3, 0.0, 15, BLUE);
            
        }

        #[allow(dead_code)]
        pub fn create_curve(
            &mut self,
            center: Vec2,
            radius: f32,
            thickness: f32,
            start_angle: f32,
            end_angle: f32,
            steps: usize,
            color: Color,
        ) {
            if steps == 0 { return; }
    
            let angle_step = (end_angle - start_angle) / steps as f32;
            for i in 0..steps {
                let angle = start_angle + i as f32 * angle_step;
    
                // Direction vector along the arc
                let dir = vec2(angle.cos(), angle.sin());
                // Center of rectangle: radius + half thickness outward
                let rect_center = center + dir * (radius + thickness * 0.5);
    
                // Rectangle size
                let arc_length = (radius + thickness) * angle_step; // length along arc
                let rect_width = arc_length * 1.05; // slight overlap (5% more)
                let rect_size = vec2(rect_width, thickness);
    
                // Rotation (tangent to the curve)
                let rotation = angle + std::f32::consts::FRAC_PI_2;
    
                // Push rectangle into physics objects
                self.colliders.push(StaticBody::new_rectangle(
                    rect_center,
                    rect_size,
                    rotation,
                    color,
                ));
            }
        }
    }
}
