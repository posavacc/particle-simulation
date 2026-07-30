use macroquad::{prelude::*};

struct Particle {
    pos: Vec2,
    vel: Vec2,
    rest: f32,

    mass: f32,
    inv_mass: f32,

    rad: f32,
    col: Color,
}

const WIDTH:  i32 = 1280;
const HEIGHT: i32 = 720;
const SCALE: f32 = 100.0;

const GRAVITY: Vec2 = Vec2 { x: 0.0, y: -9.81 };

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

    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        let dt = get_frame_time();

        let sub_dt = dt / SUBSTEPS as f32;
        for _ in 0..SUBSTEPS {
            update(&mut particles, sub_dt);
        }

        clear_background(BLACK);
        draw(&particles);

        next_frame().await
    }
}

fn update(particles: &mut [Particle], dt: f32) {
    for i in 0..particles.len() {
        for j in i+1..particles.len() {
            let (first, last) = particles.split_at_mut(j);

            let p1 = &mut first[i];
            let p2 = &mut last[0];

            particle_collision(p1, p2);
        }
    }

    for mut part in particles {
        bounds_collision(&mut part);
        integrate(&mut part, dt);
    }
}

fn integrate(p: &mut Particle, dt: f32) {
    let accel = GRAVITY;

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

            p1.vel -= j / p1.mass;
            p2.vel += j / p1.mass;
        }
    }
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
