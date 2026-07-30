use macroquad::prelude::*;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    rest: f32,

    rad: f32,
    col: Color,
}

const WIDTH:  i32 = 1280;
const HEIGHT: i32 = 720;
const SCALE: f32 = 100.0;

const GRAVITY: Vec2 = Vec2 { x: 0.0, y: -9.81 };

#[macroquad::main(window_conf())]
async fn main() {

    let particle0 = Particle { pos: vec2(0.0, 0.0), vel: vec2(10.0, 10.0), rest: 0.7, rad: 0.1, col: BLUE };
    let particle1 = Particle { pos: vec2(2.0, 0.0), vel: vec2(0.0, 0.0), rest: 0.7, rad: 0.1, col: YELLOW };

    let mut particles = vec![particle1, particle0];

    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        let dt = get_frame_time();

        for mut part in &mut particles {
            bounds_collision(&mut part);
            integrate(&mut part, dt);

        }


        clear_background(BLACK);
        draw(&particles);

        next_frame().await
    }
}

fn integrate(p: &mut Particle, dt: f32) {
    let accel = GRAVITY;

    p.vel += accel * dt;
    p.pos += p.vel * dt;
}

fn bounds_collision(p: &mut Particle) {
    let right  = WIDTH  as f32 / (SCALE * 2.0);
    let left   = -right;
    let top    = HEIGHT as f32 / (SCALE * 2.0);
    let bottom = -top;

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
