use macroquad::{prelude::*};

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub rest: f32,

    pub mass: f32,
    pub inv_mass: f32,

    pub rad: f32,
    pub col: Color,
}
