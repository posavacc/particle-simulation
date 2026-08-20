use macroquad::math::Vec2;

use crate::particle::Particle;
use crate::render::{WIDTH, HEIGHT, SCALE};

struct Bounds {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

pub struct Physics {
    gravity: Vec2,
    bounds: Bounds,
}

impl Physics {
    pub fn new() -> Self {
        let right  = WIDTH  as f32 / (SCALE * 2.0);
        let left   = -right;
        let top    = HEIGHT as f32 / (SCALE * 2.0);
        let bottom = -top;

        let bounds = Bounds { top, bottom, left, right };

        Self {
            gravity: Vec2 { x: 0.0, y: -9.81 },
            bounds: bounds,
        }
    }

    pub fn update(&self, particles: &mut [Particle], dt: f32) {
        for part in &mut *particles {
            part.accel = Vec2::ZERO;
        }

        for part in &mut *particles {
            self.integrate(part, dt);
        }

        for part in &mut *particles {
            self.bounds_collision(part);
        }

        for i in 0..particles.len() {
            for j in i+1..particles.len() {
                let (first, last) = particles.split_at_mut(j);

                let p1 = &mut first[i];
                let p2 = &mut last[0];

                Self::particle_collision(p1, p2);
            }
        }
    }

    fn integrate(&self, p: &mut Particle, dt: f32) {
        p.accel += self.gravity;

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

    fn bounds_collision(&self, p: &mut Particle) {
        let right  = self.bounds.right;
        let left   = self.bounds.left;
        let top    = self.bounds.top;
        let bottom = self.bounds.bottom;

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
}
