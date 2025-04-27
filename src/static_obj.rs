

pub mod static_obj {
    use macroquad::prelude::*;

    use crate::{physics_obj::physics_obj::PhysicsBody, rotate_vec2};

    #[allow(dead_code)]
    #[derive(Default)]
    pub enum StaticBody {
        Rectangle {
            position: Vec2,
            rotation: f32,
            dimensions: Vec2,
            color: Color,
        },
        Circle {
            position: Vec2,
            radius: f32,
            color: Color,
        },
        #[default]
        Empty 
    }

    impl StaticBody {
        #[allow(dead_code)]
        pub fn new_rectangle( position: Vec2, dimensions: Vec2, rotation: f32, color: Color) -> StaticBody {
            StaticBody::Rectangle { 
                position, 
                rotation, 
                dimensions, 
                color 
            }
        }

        #[allow(dead_code)]
        pub fn new_circle( position: Vec2, radius: f32, color: Color) -> StaticBody {
            StaticBody::Circle { 
                position, 
                radius, 
                color 
            }
        }

        #[allow(dead_code)]
        pub fn draw(&self) {
            match self {
                StaticBody::Rectangle { position, rotation, dimensions: size, color  } => {
                     draw_rectangle_ex(position.x, position.y, size.x, size.y, DrawRectangleParams { 
                        offset: vec2(0.5, 0.5), rotation: *rotation, color: *color
                    });
                },
                StaticBody::Circle { position, radius, color } => {
                    draw_circle(position.x, position.y, *radius, *color);
                },
                StaticBody::Empty => ()
            }
        }

        #[allow(dead_code)]
        //Returns (Collision point, collision normal, penetration)
        pub fn collision_check(&self, obj: &PhysicsBody) -> Option<(Vec2, Vec2, f32)> {
            match self {
                StaticBody::Circle { position, radius, color: _ } => {
                    let displacement: Vec2 = obj.position - *position;
                    let distance_squared: f32 = displacement.length_squared();

                    //If too far away return nothing
                    if distance_squared > (radius + obj.radius) * (radius + obj.radius) {
                        return None
                    }
                    //If inside object just don't do anything right now. Better than undefined behaviour
                    if distance_squared == 0.0 { return None; }

                    let distance: f32 = distance_squared.sqrt();
                    let normal: Vec2 = displacement / distance;
                    Some((
                       (normal * *radius) + *position,
                       normal,
                       radius + obj.radius - distance
                    ))
                },
                StaticBody::Rectangle { position, rotation, dimensions, .. } => {
                    let local_object_position = rotate_vec2(obj.position - *position, -rotation);

                    // Check for a potential collision
                    if let Some(mut out) = StaticBody::rectangle_collision_local(*dimensions, local_object_position, obj.radius) {
                        // Rotate the contact point and the normal back to world space
                        out.0 = rotate_vec2(out.0, *rotation) + *position;
                        out.1 = rotate_vec2(out.1, *rotation);
                        Some(out)
                    } else {
                        None
                    }
                },
                StaticBody::Empty => None
            }
        }

        fn rectangle_collision_local(dimensions: Vec2, obj_position: Vec2, obj_radius: f32) -> Option<(Vec2, Vec2, f32)> {
            
            let displacement: Vec2 = obj_position;
            let half_size: Vec2 = dimensions / 2.0;
            let centre_to_inside = displacement.abs() - half_size;

            //If not possible to be inside skip most of the code already.
            if centre_to_inside.x > obj_radius || centre_to_inside.y > obj_radius {
                return None
            }

            //Corner case
            if centre_to_inside.x > 0.0 && centre_to_inside.y > 0.0 {
                //Exit early if doesn't intersect
                let distance_sqr = centre_to_inside.length_squared();
                if distance_sqr > obj_radius.powi(2) {
                    return None;
                }

                let distance = distance_sqr.sqrt();
                let sign = displacement.signum();
                let contact_point = half_size * sign;
                let normal = sign * centre_to_inside.normalize();
                let penetration = obj_radius - distance;
                
                Some((contact_point, normal, penetration))
            }
            //Horizontal intersection resolving
            else if centre_to_inside.y < centre_to_inside.x {
                let normal = vec2(displacement.x.signum(), 0.0);
                let contact_point = vec2(half_size.x * normal.x, displacement.y);
                let penetration = obj_radius - centre_to_inside.x;
                
                Some((contact_point, normal, penetration))
            }
            //Vertical intersection resolving
            else {
                let normal = vec2(0.0, displacement.y.signum());
                let contact_point = vec2(displacement.x, half_size.y * normal.y);
                let penetration = obj_radius - centre_to_inside.y;

                Some((contact_point, normal, penetration))
            }
        }
    }
}