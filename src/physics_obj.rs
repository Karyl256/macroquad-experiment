pub mod physics_obj {
    const BOUNCINESS: f32 = 1.0;

    use macroquad::prelude::*;

    use crate::{game_engine::GRAVITY, static_obj::static_obj::StaticObj};

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
        pub fn calculate_energy(&self, bottom_y: f32) -> f32 {
            (self.velocity.length_squared() / 2.0) + (bottom_y - self.position.y) * GRAVITY.y
        }

        #[allow(dead_code)]
        pub fn run_physics(&mut self, dt: f32, list: &Vec<StaticObj>, debug_queue: &mut Vec<Vec2>) {
            #[allow(unused_mut)]
            let mut acceleration = GRAVITY;

            self.velocity += acceleration * dt;
            self.position += self.velocity * dt;

            if self.position.y + self.radius > screen_height() {
                self.velocity.y = -self.velocity.y;
                self.position.y = screen_height() - self.radius;
            }

            for obj in list {
                let contact = obj.find_collision_point(self);
                match contact {
                    Some(c) => { 
                        debug_queue.push(c.0);
                        debug_queue.push((c.1 * c.2) + self.position);

                        self.position += c.1 * c.2;
                        let velocity_dot = self.velocity.dot(c.1);
                        if velocity_dot < 0.0 {
                            self.velocity = self.velocity - (1.0 + BOUNCINESS) * velocity_dot * c.1;
                        }
                    },
                    None => {}
                }
            }
        }
    }
}
