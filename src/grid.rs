use macroquad::math::Vec2;

use crate::render::{WIDTH, HEIGHT, SCALE, reverse_projection};

pub struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(mut grid_length: i32) -> Self {
        while WIDTH % grid_length != 0 {
           grid_length += 1;
        }

        let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
        let grid_height = (grid_length as f32 / aspect_ratio).round() as u32;

        let size = WIDTH as f32 / (SCALE * grid_length as f32);
        let top_left = reverse_projection(Vec2::ZERO, SCALE);

        let mut cells = Vec::new();

        for i in 0..grid_height {
            for j in 0..grid_length {
                let x = (top_left.x + size * 1.0 * j as f32) + size * 0.5;
                let y = (top_left.y - size * 1.0 * i as f32) - size * 0.5;

                let cell = Cell {
                    pos: Vec2 { x, y },
                };

                cells.push(cell);
            }
        }

        Self {
            cells: Vec::new()
        }
    }
}

struct Cell {
    pos: Vec2,
}
