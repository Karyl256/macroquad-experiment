

pub mod static_obj {
    use std::f32::consts::PI;

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
        Curve {
            center: Vec2,
            radius: f32,
            angle_start: f32,
            angle_end: f32,
            render: Vec<StaticBody>,
        },
        Flipper {
            origin: Vec2,
            offset: Vec2,
            dimensions: Vec2,
            current_rotation: f32,
            rotation_max: f32,
            rotation_min: f32,
            angular_velocity: f32,
            color: Color,
        },
        #[default]
        Empty 
    }

    impl StaticBody {
        #[allow(dead_code)]
        pub fn new_rectangle(position: Vec2, dimensions: Vec2, rotation: f32, color: Color) -> StaticBody {
            StaticBody::Rectangle { 
                position, 
                rotation, 
                dimensions, 
                color 
            }
        }

        #[allow(dead_code)]
        pub fn new_circle(position: Vec2, radius: f32, color: Color) -> StaticBody {
            StaticBody::Circle { 
                position, 
                radius, 
                color 
            }
        }
        
        #[allow(dead_code)]
        pub fn new_curve(center: Vec2, radius: f32, thickness: f32, angle_start: f32, angle_end: f32, steps: usize, color: Color) -> StaticBody {
            let mut out = StaticBody::Curve { center, radius, angle_start, angle_end, render: Vec::new() };

            let mut angle_step = angle_end - angle_start;
            if angle_step < 0.0 { angle_step += 2.0 * PI; }
            angle_step = angle_step / steps as f32;

            for i in 0..steps {
                let angle = angle_start + i as f32 * angle_step;
    
                // Direction vector along the arc
                let dir = vec2(angle.cos(), angle.sin());
                // Center of rectangle: radius + half thickness outward
                let rect_center = center + dir * (radius + thickness * 0.5);
    
                // Rectangle size
                let arc_length = (radius + thickness.max(0.0)) * angle_step; // length along arc
                let rect_width = arc_length * 1.05; // slight overlap (5% more)
                let rect_size = vec2(rect_width, thickness);
    
                // Rotation (tangent to the curve)
                let rotation = angle + std::f32::consts::FRAC_PI_2;
    
                // Push rectangle into physics objects
                if let StaticBody::Curve { render, .. } = &mut out {
                    render.push(StaticBody::new_rectangle(
                    rect_center,
                    rect_size,
                    rotation,
                    color,
                ));}
            }
            out
        }

        #[allow(dead_code)]
        pub fn new_flipper (origin: Vec2, offset: Vec2, dimensions: Vec2, rotation_min: f32, rotation_max: f32, color: Color) -> StaticBody {
            StaticBody::Flipper { 
                origin, 
                offset, 
                dimensions, 
                current_rotation: rotation_min, 
                rotation_max, rotation_min, 
                angular_velocity: 0.0, 
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
                StaticBody::Curve { render, .. } => {
                    for obj in render {
                        if let StaticBody::Curve {..} = obj {
                            panic!("Why tf is there recursion in draw of Curve???");
                        }
                        obj.draw();
                    }
                }
                StaticBody::Flipper { origin, offset, dimensions, current_rotation, color, .. } => {
                    let rotated_offset = rotate_vec2(*offset, *current_rotation);
                    let position = *origin + rotated_offset;

                    draw_rectangle_ex(position.x, position.y, dimensions.x, dimensions.y, DrawRectangleParams { 
                        offset: vec2(0.5, 0.5), rotation: *current_rotation, color: *color
                    });
                }
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
                    //If inside centre just don't do anything right now. Better than undefined behaviour
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
                StaticBody::Curve { center, radius, render: _, angle_start, angle_end } => {
                    let displacement: Vec2 = obj.position - *center;
                    let distance_to_center: f32 = displacement.length();

                    //Exit early if too far or too close
                    if distance_to_center > radius + obj.radius || distance_to_center < radius - obj.radius {
                        return None;
                    }

                    //Check can it touch inside part or edges the curve
                    let is_angle_between: bool = {
                        let angle_to_obj = displacement.to_angle().rem_euclid(std::f32::consts::TAU);
                        let angle_start = angle_start.rem_euclid(std::f32::consts::TAU);
                        let angle_end = angle_end.rem_euclid(std::f32::consts::TAU);

                        if angle_start < angle_end {
                            angle_to_obj <= angle_end && angle_to_obj >= angle_start
                        }
                        else {
                            angle_to_obj <= angle_end || angle_to_obj >= angle_start
                        }
                    };
                    
                    //If object touches arc's inside then
                    if is_angle_between {
                        let collision_point: Vec2 = displacement / distance_to_center * *radius + *center;
                        let collision_point_displacement: Vec2 = obj.position - collision_point;
                        let penetration: f32 = obj.radius - collision_point_displacement.length();

                        Some((collision_point, collision_point_displacement.normalize(), penetration))
                    }
                    //Edges
                    else {
                        let start: Vec2 = *center + rotate_vec2(vec2(*radius, 0.0), *angle_start);
                        let end: Vec2 = *center + rotate_vec2(vec2(*radius, 0.0), *angle_end);

                        let start_distance = start.distance(obj.position);
                        let end_distance = end.distance(obj.position);

                        if start_distance < end_distance {
                            let start_displacement = obj.position - start;
                            if start_displacement.length() < obj.radius {
                                Some((start, start_displacement.normalize(), obj.radius - start_displacement.length()))
                            }
                            else {
                                None
                            }
                        }
                        else {
                            let end_displacement = obj.position - end;
                            if end_displacement.length() < obj.radius {
                                Some((end, end_displacement.normalize(), obj.radius - end_displacement.length()))
                            }
                            else {
                                None 
                            }
                        }
                    }
                },
                StaticBody::Flipper { origin, offset, dimensions, current_rotation, color, .. } => {
                    let position = *origin + rotate_vec2(*offset, *current_rotation);

                    StaticBody::new_rectangle(position, *dimensions, *current_rotation, *color).collision_check(obj)
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