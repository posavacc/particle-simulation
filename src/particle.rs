use macroquad::{prelude::*};

pub struct Particle {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub accel: Vec2,
    pub force: Vec2,

    pub mass: f32,
    pub rest: f32,

    pub idx: i32,

    pub rad: f32,
    pub col: Color,
}

impl Particle {
    pub fn new(pos: Vec2, rest: f32, radius: f32, color: Color, idx: i32) -> Self {
        let mass = radius * radius * 1.0;

        Self {
            pos: pos,
            prev_pos: pos,
            accel: Vec2::ZERO,
            force: Vec2::ZERO,
            mass: mass,
            rest: rest,
            idx: idx,
            rad: radius,
            col: color
        }
    }

    pub fn reset_accel(&mut self) {
        self.accel = Vec2::ZERO;
    }
}
