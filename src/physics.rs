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

        if p.pos.x - right + r >= 0.0 {
            let vx = p.pos.x - p.prev_pos.x;
            p.pos.x = right - r;
            p.prev_pos.x = p.pos.x + vx * rest;

        } else if p.pos.x - left - r <= 0.0 {
            let vx = p.pos.x - p.prev_pos.x;
            p.pos.x = left + r;
            p.prev_pos.x = p.pos.x + vx * rest;
        }

        if p.pos.y - top + r >= 0.0 {
            let vy = p.pos.y - p.prev_pos.y;
            p.pos.y = top - r;
            p.prev_pos.y = p.pos.y + vy * rest;

        } else if p.pos.y - bottom - r <= 0.0 {
            let vy = p.pos.y - p.prev_pos.y;
            p.pos.y = bottom + r;
            p.prev_pos.y = p.pos.y + vy * rest;
        }
    }
}
