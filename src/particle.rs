use macroquad::{prelude::*};

pub struct Particle {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub accel: Vec2,
    pub rest: f32,

    pub rad: f32,
    pub col: Color,
}

impl Particle {
    pub fn new(pos: Vec2, rest: f32, color: Color) -> Self {
        Self {
            pos: pos,
            prev_pos: pos,
            accel: vec2(0.0, 0.0),
            rest: rest,
            rad: 0.10,
            col: color
        }
    }
}
