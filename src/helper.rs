

pub mod helper {
    use macroquad::math::Vec2;
    
    pub fn rotate_vec2(v: Vec2, angle: f32) -> Vec2 {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        Vec2::new(
            v.x * cos_theta - v.y * sin_theta,
            v.x * sin_theta + v.y * cos_theta,
        )
    }
}

#[macro_export]
macro_rules! flipper {
    ($self:ident, $pos:expr, $pivot:expr, $size:expr, $angle:expr, $speed:expr, $color:expr) => {
        $self.colliders.push(StaticBody::new_flipper($pos, $pivot, $size, $angle, $speed, $color));
    };
}

#[macro_export]
macro_rules! rect {
    ($self:ident, $pos:expr, $size:expr, $angle:expr, $color:expr) => {
        $self.colliders.push(StaticBody::new_rectangle($pos, $size, $angle, $color));
    };
}

#[macro_export]
macro_rules! curve {
    ($self:ident, $center:expr, $radius:expr, $width:expr, $start:expr, $end:expr, $segments:expr, $color:expr) => {
        $self.colliders.push(StaticBody::new_curve($center, $radius, $width, $start, $end, $segments, $color));
    };
}

#[macro_export]
macro_rules! circle {
    ($self:ident, $center:expr, $radius:expr, $color:expr) => {
        $self.colliders.push(StaticBody::new_circle($center, $radius, $color));
    };
}
