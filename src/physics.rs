use macroquad::math::Vec2;

use crate::particle::Particle;
use crate::WIDTH;
use crate::HEIGHT;
use crate::SCALE;

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
        for mut part in &mut *particles {
            self.bounds_collision(&mut part);
            self.integrate(&mut part, dt);
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
        let accel = self.gravity;

        p.vel += accel * dt;
        p.pos += p.vel * dt;
    }

    fn particle_collision(p1: &mut Particle, p2: &mut Particle) {
        //let rest = (p1.rest * p2.rest).sqrt();

        let dst = p1.pos.distance(p2.pos);

        let overlap = p1.rad + p2.rad - dst;

        if overlap > 0.0 && dst != 0.0 {
            let n = (p2.pos - p1.pos) / dst;

            let rel_v = p2.vel - p1.vel;
            let n_v = rel_v.dot(n);

            if n_v <= 0.0 {
                let j = -(1.0 + p1.rest) * n_v / (p1.inv_mass + p2.inv_mass);
                let j = n * j;

                p1.pos -= overlap * 0.5 * n;
                p2.pos += overlap * 0.5 * n;

                p1.vel -= j * p1.inv_mass;
                p2.vel += j * p2.inv_mass;
            }
        }
    }

    fn bounds_collision(&self, p: &mut Particle) {
        let right  = self.bounds.right;
        let left   = self.bounds.left;
        let top    = self.bounds.top;
        let bottom = self.bounds.bottom;

        let r = p.rad;

        if p.pos.x >= right - r {
            p.pos.x = right - r;

            if p.vel.x > 0.0 {
                p.vel.x = -p.vel.x * p.rest;
            }
        } else if p.pos.x <= left + r {
            p.pos.x = left + r;

            if p.vel.x < 0.0 {
                p.vel.x = -p.vel.x * p.rest;
            }
        }

        if p.pos.y >= top - r {
            p.pos.y = top - r;

            if p.vel.y > 0.0 {
                p.vel.y = -p.vel.y * p.rest;
            }
        } else if p.pos.y <= bottom + r {
            p.pos.y = bottom + r;

            if p.vel.y < 0.0 {
                p.vel.y = -p.vel.y * p.rest;
            }
        }

        if p.vel.x.abs() < 0.01 {
            p.vel.x = 0.0;
        }

        if p.vel.y.abs() < 0.01 {
            p.vel.y = 0.0;
        }
    }
}

