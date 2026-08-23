use macroquad::{prelude::*};
use crate::particle::Particle;

pub const WIDTH:  i32 = 1920;
pub const HEIGHT: i32 = 1080;
pub const SCALE: f32 = 200.0;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn project_to_screen(p: Vec2, scale: f32) -> Vec2 {
    Vec2 {
        x:  p.x * scale + (WIDTH  as f32 / 2.0),
        y: -p.y * scale + (HEIGHT as f32 / 2.0)
    }
}

pub fn reverse_projection(p: Vec2, scale: f32) -> Vec2 {
    Vec2 {
        x: ( p.x - (WIDTH  as f32 / 2.0)) / scale,
        y: (-p.y + (HEIGHT as f32 / 2.0)) / scale
    }
}

pub fn draw(parts: &Vec<Particle>, t: f32) {
    for p in parts {
        let radius = p.rad;

        let pos_x = lerp(p.prev_pos.x, p.pos.x, t);
        let pos_y = lerp(p.prev_pos.y, p.pos.y, t);

        let screen_pos = project_to_screen(vec2(pos_x, pos_y), SCALE);

        draw_circle(screen_pos.x, screen_pos.y, radius * SCALE, p.col);
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "particle simulation".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: true,
        platform: miniquad::conf::Platform {
           swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}
