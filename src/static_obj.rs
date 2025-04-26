

pub mod static_obj {
    use macroquad::prelude::*;

    use crate::physics_obj::physics_obj::PhysicsObj;

    #[allow(dead_code)]
    #[derive(Default)]
    pub enum StaticObj {
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

    impl StaticObj {
        #[allow(dead_code)]
        pub fn draw(&self) {
            match self {
                StaticObj::Rectangle { position: pos, rotation: rot, dimensions: size, color : c } => {
                     draw_rectangle_ex(pos.x, pos.y, size.x, size.y, DrawRectangleParams { 
                        offset: vec2(0.5, 0.5), rotation: *rot, color: *c 
                    });
                },
                StaticObj::Circle { position: pos, radius: r, color: c } => {
                    draw_circle(pos.x, pos.y, *r, *c);
                },
                StaticObj::Empty => ()
            }
        }

        #[allow(dead_code)]
        pub fn find_collision_point(&self, obj: &PhysicsObj) -> Option<(Vec2, Vec2, f32)> {
            match self {
                StaticObj::Circle { position, radius, color: _ } => {
                    let displacement: Vec2 = obj.position - *position;
                    let distance_squared: f32 = displacement.length_squared();

                    //If too far return nothing
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
                StaticObj::Rectangle { position, rotation, dimensions, color: _ } => {
                    let displacement: Vec2 = obj.position - *position;

                    None
                },
                StaticObj::Empty => None
            }
        }
    }
}