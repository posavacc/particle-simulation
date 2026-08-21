use macroquad::math::Vec2;

use crate::particle::Particle;

const GRAVITY: Vec2 = Vec2 { x: 0.0, y: -9.81 };

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

pub fn update(particles: &mut [Particle], dt: f32, bounds: &Bounds) {
    for part in &mut *particles {
        integrate(part, dt);
    }

    for part in &mut *particles {
        bounds_collision(part, &bounds);
    }

    for i in 0..particles.len() {
        for j in i+1..particles.len() {
            let (first, last) = particles.split_at_mut(j);

            let p1 = &mut first[i];
            let p2 = &mut last[0];

            particle_collision(p1, p2);
        }
    }
}

fn integrate(p: &mut Particle, dt: f32) {
    p.accel += GRAVITY;

    let temp_pos = p.pos;
    p.pos = p.pos * 2.0 - p.prev_pos + p.accel * dt * dt;
    p.prev_pos = temp_pos;
}

fn particle_collision(p1: &mut Particle, p2: &mut Particle) {
    let dst = p1.pos.distance(p2.pos);

    let overlap = p1.rad + p2.rad - dst;

    if overlap > 0.0 && dst != 0.0 {
        let n = (p2.pos - p1.pos) / dst;

        p1.pos -= overlap * 0.5 * n;
        p2.pos += overlap * 0.5 * n;
    }
}

fn bounds_collision(p: &mut Particle, bounds: &Bounds) {
    let right  = bounds.max.x;
    let left   = bounds.min.x;
    let top    = bounds.max.y;
    let bottom = bounds.min.y;

    let rest = p.rest;
    let r = p.rad;

    let overlap_right = (p.pos.x + r) - right;
    let overlap_left = (p.pos.x - r) - left;
    let overlap_bottom = (p.pos.y - r) - bottom;
    let overlap_top = (p.pos.y + r) - top;

    if overlap_right >= 0.0 {
        p.pos.x -= overlap_right * (1.0 + rest);

    } else if overlap_left <= 0.0 {
        p.pos.x -= overlap_left * (1.0 + rest);
    }

    if overlap_top >= 0.0 {
        p.pos.y -= overlap_top * (1.0 + rest);

    } else if overlap_bottom <= 0.0 {
        p.pos.y -= overlap_bottom * (1.0 + rest);
    }
}
