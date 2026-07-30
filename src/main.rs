use macroquad::prelude::*;

const WIDTH:  i32 = 1280;
const HEIGHT: i32 = 720;

const GRAVITY: Vec2 = Vec2 { x: 0.0, y: -9.81 };

struct Particle {
    pos: Vec2,
    vel: Vec2,
}

fn integrate(p: &mut Particle, dt: f32) {
    let accel = GRAVITY;

    p.vel += accel * dt;
    p.pos += p.vel * dt;
}

#[macroquad::main(window_conf())]
async fn main() {

    let mut particle = Particle { pos: vec2(0.0, 0.0), vel: vec2(0.0, 0.0) };


    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        let dt = get_frame_time();

        integrate(&mut particle, dt);

        let screen_point = project_to_screen(particle.pos);

        clear_background(BLACK);

        draw_circle(screen_point.x, screen_point.y, 10.0, BLUE);

        next_frame().await
    }
}

fn project_to_screen(p: Vec2) -> Vec2 {
    let x =  p.x + (WIDTH  as f32 / 2.0);
    let y = -p.y + (HEIGHT as f32 / 2.0);

    return Vec2 { x, y };
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
