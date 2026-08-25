use macroquad::math::Vec2;

use crate::particle::Particle;
use crate::grid::Grid;

pub const GRAVITY: Vec2 = Vec2 { x: 0.0, y: -9.81 * 1.0 };

pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl Bounds {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            min: Vec2 { x: left, y: bottom },
            max: Vec2 { x: right, y: top },
        }
    }
}

pub fn update(particles: &mut Vec<Particle>, dt: f32, grid: &mut Grid, bounds: &Bounds) {
    for part in &mut *particles {
        integrate(part, dt);
    }

    for part in &mut *particles {
        bounds_collision(part, &bounds);
    }

    grid.check_all_cell_collision(particles);
}

fn integrate(p: &mut Particle, dt: f32) {
    p.accel += GRAVITY;

    let temp_pos = p.pos;
    p.pos = p.pos * 2.0 - p.prev_pos + p.accel * dt * dt;
    p.prev_pos = temp_pos;
}

pub fn particle_collision(p1: &mut Particle, p2: &mut Particle) {
    let dst_sqrd = p1.pos.distance_squared(p2.pos);
    let radii = p1.rad + p2.rad;
    let epsilon = 0.01;

    if dst_sqrd < radii * radii && dst_sqrd > epsilon * epsilon {
        let dst = dst_sqrd.sqrt();

        let overlap = radii - dst;
        let n = (p2.pos - p1.pos) / dst;

        let mass_weight = p2.mass / (p1.mass + p2.mass);
        let relaxation = 0.4;

        p1.pos -= overlap * n * mass_weight * relaxation;
        p2.pos += overlap * n * (1.0 - mass_weight) * relaxation;
    }
}

fn bounds_collision(p: &mut Particle, bounds: &Bounds) {
    let r = p.rad;

    let right  = bounds.max.x - r;
    let left   = bounds.min.x + r;
    let top    = bounds.max.y - r;
    let bottom = bounds.min.y + r;

    let rest = p.rest;

    if p.pos.x >= right {
        let velocity_x = p.pos.x - p.prev_pos.x;

        p.pos.x = right;

        p.prev_pos.x = p.pos.x + velocity_x * rest;

    } else if p.pos.x <= left {
        let velocity_x = p.pos.x - p.prev_pos.x;

        p.pos.x = left;

        p.prev_pos.x = p.pos.x + velocity_x * rest;
    }

    if p.pos.y <= bottom {
        let velocity_y = p.pos.y - p.prev_pos.y;

        p.pos.y = bottom;

        p.prev_pos.y = p.pos.y + velocity_y * rest;

    } else if p.pos.y >= top {
        let velocity_y = p.pos.y - p.prev_pos.y;

        p.pos.y = top;

        p.prev_pos.y = p.pos.y + velocity_y * rest;
    }
}
