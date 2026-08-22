use macroquad::math::Vec2;

use crate::render::{WIDTH, HEIGHT, SCALE, reverse_projection};

pub struct Grid {
    cells: Vec<Cell>,
    cell_size: f32,
    rows: u32,
    collumns: u32,
}

impl Grid {
    pub fn new(grid_length: u32) -> Self {
        let mut collumns = grid_length;
        while WIDTH as u32 % collumns != 0 {
            collumns += 1;
        }
        let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
        let rows: u32 = (collumns as f32 / aspect_ratio).round() as u32;

        Self {
            cells: Vec::new(),
            cell_size: 0.5,
            rows: rows,
            collumns: collumns,
        }
    }

    pub fn get_index(&self, pos: Vec2) -> (usize, usize) {
        let index_x: usize = (pos.x / self.cell_size as f32).floor() as usize;
        let index_y: usize = (pos.y / self.cell_size as f32).floor() as usize;

        (index_x, index_y)
    }
}

struct Cell {
    pos: Vec2,
}
