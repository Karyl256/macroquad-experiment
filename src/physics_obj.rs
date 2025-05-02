pub mod physics_obj {
    const BOUNCINESS: f32 = 0.6;

    use macroquad::prelude::*;

    use crate::{game_engine::GRAVITY, static_obj::static_obj::StaticBody};

    #[derive(Default)]
    pub struct PhysicsBody {
        pub position: Vec2,
        pub velocity: Vec2,
        pub radius: f32,
    }

    impl PhysicsBody {

        #[allow(dead_code)]
        pub fn empty() -> PhysicsBody {
            PhysicsBody {
                position: Vec2::new(0.0, 0.0),
                velocity: Vec2::new(0.0, 0.0),
                radius: 1.0,
            }
        }

        #[allow(dead_code)]
        pub fn new(position: Vec2, velocity: Vec2, radius: f32) -> PhysicsBody {
            PhysicsBody {
                position,
                velocity,
                radius,
            }
        }
        #[allow(dead_code)]
        pub fn calculate_energy(&self, bottom_y: f32) -> f32 {
            (self.velocity.length_squared() / 2.0) + (bottom_y - self.position.y) * GRAVITY.y
        }

        pub fn update_physics(&mut self, dt: f32, colliders: &Vec<StaticBody>, debug_draw_points: &mut Vec<(Vec2, i32)>) {
            #[allow(unused_mut)]
            let mut acceleration = GRAVITY;

            self.velocity += acceleration * dt;
            self.position += self.velocity * dt - 0.5 * acceleration * dt * dt;

            for obj in colliders {
                // contact (collision point, collision normal, penetration_depth)
                let contact = obj.collision_check(self);
                if let Some(c) = contact {
                    debug_draw_points.push((c.0, 5));
                    debug_draw_points.push(((c.1 * c.2) + self.position, 5));

                    // Compute relative velocity at contact point
                    let obj_velocity_at_point = match obj {
                        StaticBody::Flipper {
                            origin,
                            angular_velocity,
                            ..
                        } => {
                            let r = c.0 - *origin;
                            Vec2::new(-r.y, r.x) * *angular_velocity
                        },
                        StaticBody::Circle { impact_force, .. } => c.1 * *impact_force,
                        StaticBody::Rectangle { impact_force, .. } => c.1 * *impact_force,
                        _ => Vec2::ZERO,
                    };
                    
                    let relative_velocity = self.velocity - obj_velocity_at_point;

                    let velocity_dot = relative_velocity.dot(c.1);
                    if velocity_dot < 0.0 {
                        let impulse = (1.0 + BOUNCINESS) * velocity_dot;

                        self.velocity -= impulse * c.1;
                        self.position += c.1 * c.2;
                    }

                    //self.position += c.1 * c.2;
                }
            }
        }
    }
}
