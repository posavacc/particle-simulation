use macroquad::{prelude::*};
use crate::particle::Particle;
use crate::physics::Physics;

mod particle;
mod physics;

const WIDTH:  i32 = 1280;
const HEIGHT: i32 = 720;
const SCALE: f32 = 100.0;

const PARTICLE_COUNT: i32 = 300;
const SUBSTEPS: i32 = 8;

#[macroquad::main(window_conf())]
async fn main() {

    let mut particles = Vec::new();

    for i in 0..PARTICLE_COUNT {
        let a: f32 = ::rand::random_range(-05.0..05.0);
        let b: f32 = ::rand::random_range(-05.0..05.0);

        let mut color = YELLOW;
        if i % 2 == 0 {
            color = BLUE;
        }

        let p = Particle {
            pos: vec2(a, b),
            vel: vec2(0.0, 0.0),
            rest: 0.7,
            mass: 1.0,
            inv_mass: 1.0,
            rad: 0.1,
            col: color
        };

        particles.push(p);
    }

    let sim = Physics::new();

    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        let dt = get_frame_time();

        let sub_dt = dt / SUBSTEPS as f32;
        for _ in 0..SUBSTEPS {
            sim.update(&mut particles, sub_dt);
        }

        clear_background(BLACK);
        draw(&particles);

        next_frame().await
    }
}


fn project_to_screen(p: Vec2, scale: f32) -> Vec2 {
    Vec2 {
        x:  p.x * scale + (WIDTH  as f32 / 2.0),
        y: -p.y * scale + (HEIGHT as f32 / 2.0)
    }
}

fn draw(parts: &[Particle]) {
    for p in parts {
        let radius = p.rad;
        let screen_pos = project_to_screen(p.pos, SCALE);

        draw_circle(screen_pos.x, screen_pos.y, radius * SCALE, p.col);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "particle simulation".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: true,
        ..Default::default()
    }
}
