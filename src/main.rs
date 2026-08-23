use macroquad::{prelude::*};
use particle_simulation::{particle::Particle, physics::*, render::*, grid::*};

const PARTICLE_COUNT: i32 = 2000;
const SUBSTEPS: i32 = 8;
const FIXED_DT: f32 = 1.0 / 100.0;

#[macroquad::main(window_conf())]
async fn main() {

    let mut particles: Vec<Particle> = Vec::new();

    let top_left = reverse_projection(vec2(0.0, 0.0), SCALE);
    let bottom_right = reverse_projection(vec2(WIDTH as f32, HEIGHT as f32), SCALE);
    for i in 0..PARTICLE_COUNT {
        let a = ::rand::random_range(top_left.x..bottom_right.x);
        let b = ::rand::random_range(bottom_right.y..top_left.y);

        let mut color = PURPLE;
        if i % 2 == 0 {
            color = VIOLET;
        } if i % 3 == 0 {
            color = Color::new(0.5, 0.0, 0.5, 1.0);
        }

        let radius = ::rand::random_range(0.02..0.05);
        let p = Particle::new(vec2(a, b), 0.7, radius, color, i);

        particles.push(p);
    }

    let right = reverse_projection(vec2(WIDTH as f32, 0.0), SCALE).x;
    let left = reverse_projection(vec2(0.0, 0.0), SCALE).x;
    let top = reverse_projection(vec2(0.0, 0.0), SCALE).y;
    let bottom = reverse_projection(vec2(0.0, HEIGHT as f32), SCALE).y;

    let bounds = Bounds::new(left, right, top, bottom);

    let mut grid = Grid::new(0.1, &bounds);

    let mut accumulator = 0.0;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        let dt = get_frame_time();
        accumulator += dt;

        while accumulator > FIXED_DT  {
            let sub_dt = FIXED_DT / SUBSTEPS as f32;
            for _ in 0..SUBSTEPS {
                for p in &mut particles {
                    p.reset_accel();
                }
                handle_input(&mut particles);

                grid.calculate_cells(&particles, &bounds);

                update(&mut particles, sub_dt, &mut grid, &bounds);

                grid.reset_grid();
            }

            accumulator -= FIXED_DT;
        }


        clear_background(BLACK);

        let t = accumulator / FIXED_DT;
        draw(&particles, t);
        draw_fps();

        next_frame().await
    }
}

fn handle_input(particles: &mut Vec<Particle>) {
    if is_mouse_button_down(MouseButton::Left) {
        let (x, y) = mouse_position();
        let mouse_pos = Vec2 { x, y };
        let mouse_pos = reverse_projection(mouse_pos, SCALE);
        for p in &mut *particles {
            let dst = mouse_pos.distance(p.pos);
            if dst < 1.2 && dst != 0.0 {
                let dir = (mouse_pos - p.pos) / dst;
                let force = 70.0;
                p.accel.x += force * dir.x;
                p.accel.y += force * dir.y + GRAVITY.y;
            }
        }
    }
    if is_mouse_button_down(MouseButton::Right) {
        let (x, y) = mouse_position();
        let mouse_pos = Vec2 { x, y };
        let mouse_pos = reverse_projection(mouse_pos, SCALE);
        for p in &mut *particles {
            let dst = mouse_pos.distance(p.pos);
            if dst < 0.7 && dst != 0.0 {
                let dir = (mouse_pos - p.pos) / dst;
                let force = 100.0;
                p.accel.x -= force * dir.x;
                p.accel.y -= force * dir.y - GRAVITY.y;
            }
        }
    }

    let force = 20.0;
    if is_key_down(KeyCode::Left) {
        for p in &mut *particles {
            p.accel = Vec2::new(-force, 0.0);
        }
    }
    if is_key_down(KeyCode::Right) {
        for p in &mut *particles {
            p.accel = Vec2::new(force, 0.0);
        }
    }
    if is_key_down(KeyCode::Up) {
        for p in &mut *particles {
            p.accel = Vec2::new(0.0, force - GRAVITY.y);
        }
    }
    if is_key_down(KeyCode::Down) {
        for p in &mut *particles {
            p.accel = Vec2::new(0.0, -force);
        }
    }
}
