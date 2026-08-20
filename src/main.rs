use macroquad::{prelude::*};
use particle_simulation::{particle::Particle, physics::Physics, render::*};

const PARTICLE_COUNT: i32 = 1000;
const SUBSTEPS: i32 = 8;

#[macroquad::main(window_conf())]
async fn main() {

    let mut particles: Vec<Particle> = Vec::new();

    let top_left = reverse_projection(vec2(0.0, 0.0), SCALE);
    let bottom_right = reverse_projection(vec2(WIDTH as f32, HEIGHT as f32), SCALE);
    for i in 0..PARTICLE_COUNT {
        let mut a: f32 = ::rand::random_range(-08.0..08.0);
        let mut b: f32 = ::rand::random_range(-05.0..05.0);

        for part in &particles {
            let mut dst = vec2(a, b).distance(part.pos);
            while dst < 5.0 {
                a = ::rand::random_range(top_left.x..bottom_right.x);
                b = ::rand::random_range(bottom_right.y..top_left.y);

                dst = vec2(a, b).distance(part.pos);
            }
        }

        let mut color = YELLOW;
        if i % 2 == 0 {
            color = BLUE;
        } if i % 3 == 0 {
            color = RED;
        }

        let radius = ::rand::random_range(0.06..0.14);
        let p = Particle::new(vec2(a, b), 0.7, radius, color);

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
        draw_fps();

        next_frame().await
    }
}


