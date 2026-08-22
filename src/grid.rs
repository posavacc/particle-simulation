use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::particle::{Particle};
use crate::physics::{particle_collision};

pub struct Grid {
    cells: HashMap<(i32, i32), Vec<i32>>,
    cell_size: f32,
}

impl Grid {
    pub fn new(size: f32) -> Self {
        Self {
            cells: HashMap::new(),
            cell_size: size,
        }
    }

    pub fn check_cell_collision(&self, particles: &mut Vec<Particle>, index: usize) {
        let particle = particles.get(index).unwrap();
        let (cx, cy) = self.cell_coordinate(particle.pos);
        for dx in -1..=1 {
            for dy in -1..=1 {
                let key = (cx + dx, cy + dy);
                let idx_vec = match self.cells.get(&key) {
                    Some(value) => value,
                    None => continue
                };

                for idx in idx_vec {
                    if *idx > index as i32 {
                        let (first, last) = particles.split_at_mut(*idx as usize);

                        let p1 = &mut first[index];
                        let p2 = &mut last[0];
                        particle_collision(p1, p2);
                    }
                }
            }
        }
    }

    pub fn calculate_cells(&mut self, particles: &Vec<Particle>) {
        for p in particles {
            let key = self.cell_coordinate(p.pos);
            self.cells.entry(key).or_insert_with(|| { Vec::new() }).push(p.idx);
        }
    }

    pub fn reset_grid(&mut self) {
        self.cells.clear();
    }

    fn cell_coordinate(&self, point: Vec2) -> (i32, i32) {
        let cx = (point.x / self.cell_size).floor() as i32;
        let cy = (point.y / self.cell_size).floor() as i32;

        (cx, cy)
    }
}
