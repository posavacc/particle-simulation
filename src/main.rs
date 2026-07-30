use macroquad::prelude::*;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
}

const WIDTH:  i32 = 1280;
const HEIGHT: i32 = 720;

const GRAVITY: Vec2 = Vec2 { x: 0.0, y: -9.81 };

#[macroquad::main(window_conf())]
async fn main() {

    let mut particle = Particle { pos: vec2(0.0, 0.0), vel: vec2(0.0, 0.0), radius: 0.1 };
    

    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        let dt = get_frame_time();

        integrate(&mut particle, dt);

        draw(&particle);

        next_frame().await
    }
}

fn integrate(p: &mut Particle, dt: f32) {
    let accel = GRAVITY;

    p.vel += accel * dt;
    p.pos += p.vel * dt;
}

fn project_to_screen(p: Vec2, scale: f32) -> Vec2 {
    Vec2 {
        x:  p.x * scale + (WIDTH  as f32 / 2.0),
        y: -p.y * scale + (HEIGHT as f32 / 2.0)
    }
}

fn draw(p: &Particle) {
    let radius = p.radius;
    let scale = 100.0;
    let screen_pos = project_to_screen(p.pos, scale);

    clear_background(BLACK);
    draw_circle(screen_pos.x, screen_pos.y, radius * scale, BLUE);
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
