use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::particle::{Particle};
use crate::physics::{particle_collision, Bounds};

pub struct Grid {
    pub cells: HashMap<(i32, i32), Vec<usize>>,

    cell_length: f32,
    cell_height: f32,

    columns: i32,
    rows: i32,
}

impl Grid {
    pub fn new(size: f32, bounds: &Bounds) -> Self {
        let world_width = bounds.max.x * 2.0;
        let world_height = bounds.max.y * 2.0;

        let mut columns = world_width / size;
        let mut rows = world_height / size;

        let mut length;
        let mut height;

        if !(columns.round() == columns) {
            columns = columns.round();
            length = world_width / columns;

            while world_width % length != 0.0 {
                columns -= 1.0;
                length = world_width / columns;
            }
        } else {
            length = size;
        }

        if !(rows.round() == rows) {
            rows = rows.round();
            height = world_height / rows.round();

            while world_height % height != 0.0 {
                rows -= 1.0;
                height = world_height / rows;
            }
        } else {
            height = size;
        }

        Self {
            cells: HashMap::new(),
            cell_length: length,
            cell_height: height,
            columns: columns as i32,
            rows: rows as i32,
        }
    }

    fn check_cell_collision(&mut self, cell_coords: (i32, i32), particles: &mut Vec<Particle>) {
        let (cx, cy) = cell_coords;

        let cell_vector = self.cells.get(&cell_coords);
        let cell_indicies = match cell_vector {
            Some(vector) => vector,
            None => return
        };

        for i in 0..cell_indicies.len() {
            let index = match cell_indicies.get(i) {
                Some(value) => value,
                None => continue
            };

            for dx in 0..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 1 {
                        continue;
                    }
                    let key = (cx + dx, cy + dy);
                    let idx_vec = match self.cells.get(&key) {
                        Some(value) => value,
                        None => continue
                    };

                    for idx in idx_vec {
                        if dx == 0 && dy == 0 {
                            if *idx > *index {
                                let (first, last) = particles.split_at_mut(*idx as usize);

                                let p1 = &mut first[*index as usize];
                                let p2 = &mut last[0];

                                particle_collision(p1, p2);
                            }
                        } else {
                            if *idx > *index {
                                let (first, last) = particles.split_at_mut(*idx as usize);

                                let p1 = &mut first[*index as usize];
                                let p2 = &mut last[0];

                                particle_collision(p1, p2);

                            } else if *idx < *index {
                                let (first, last) = particles.split_at_mut(*index as usize);

                                let p1 = &mut first[*idx as usize];
                                let p2 = &mut last[0];

                                particle_collision(p1, p2);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn check_all_cell_collision(&mut self, particles: &mut Vec<Particle>) {
        for cx in 0..self.columns {
            for cy in 0..self.rows {
                self.check_cell_collision((cx, cy), particles);
            }
        }
    }

    pub fn calculate_cells(&mut self, particles: &Vec<Particle>, bounds: &Bounds) {
        for i in 0..particles.len() {
            let p = match particles.get(i) {
                Some(value) => value,
                None => continue
            };
            let key = self.cell_coordinate(p.pos, bounds);
            self.cells.entry(key).or_insert_with(|| { Vec::new() }).push(i);
        }
    }

    pub fn reset_grid(&mut self) {
        for value in self.cells.values_mut() {
            value.clear();
        }
    }

    fn cell_coordinate(&self, point: Vec2, bounds: &Bounds) -> (i32, i32) {
        let cx = ((point.x  + bounds.max.x)/ self.cell_length).floor() as i32;
        let cy = ((point.y + bounds.max.y)/ self.cell_height).floor() as i32;

        (cx, cy)
    }
}
