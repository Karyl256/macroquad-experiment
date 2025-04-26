pub mod physics_obj {

    use macroquad::prelude::*;

    use crate::game_engine::GRAVITY;

    #[derive(Default)]
    pub struct PhysicsObj {
        pub position: Vec2,
        pub velocity: Vec2,
        pub radius: f32,
    }

    impl PhysicsObj {

        #[allow(dead_code)]
        pub fn empty() -> PhysicsObj {
            PhysicsObj {
                position: Vec2::new(0.0, 0.0),
                velocity: Vec2::new(0.0, 0.0),
                radius: 1.0,
            }
        }

        #[allow(dead_code)]
        pub fn new(position: Vec2, velocity: Vec2, radius: f32) -> PhysicsObj {
            PhysicsObj {
                position,
                velocity,
                radius,
            }
        }

        #[allow(dead_code)]
        pub fn run_physics(&mut self, dt: f32, _list: &Vec<PhysicsObj>) {
            #[allow(unused_mut)]
            let mut acceleration = GRAVITY;
            
            if self.position.y + self.radius > screen_height() {
                self.velocity.y = -self.velocity.y;
                self.position.y = screen_height() - self.radius;
            }

            self.velocity += acceleration * dt;
            self.position += self.velocity * dt;
            
        }
    }
}
