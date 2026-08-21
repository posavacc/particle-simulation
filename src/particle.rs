use macroquad::{prelude::*};

const GRAVITY: Vec2 = Vec2 { x: 0.0, y: -9.81 };

pub struct Particle {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub accel: Vec2,
    pub force: Vec2,
    pub rest: f32,

    pub rad: f32,
    pub col: Color,
}

impl Particle {
    pub fn new(pos: Vec2, rest: f32, radius: f32, color: Color) -> Self {
        Self {
            pos: pos,
            prev_pos: pos,
            accel: Vec2::ZERO,
            force: Vec2::ZERO,
            rest: rest,
            rad: radius,
            col: color
        }
    }

    pub fn reset_accel(&mut self) {
        self.accel = GRAVITY;
    }
}
