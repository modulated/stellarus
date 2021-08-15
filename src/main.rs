use macroquad::prelude::*;

mod body;
use body::Body;

#[macroquad::main(app_setup)]
async fn main() {    
    let mut game_running = true;
    let mut bodies: Vec<Body> = Vec::new();

    for _ in 1..100 {
        bodies.push(Body::random(100));
    }

    bodies.push(Body {
        mass: 1000,
        velocity: Vec2::new(0.0, 0.0),
        position: Vec2::new (screen_width()/2.0, screen_height()/2.0)
    });

    info!("Beggining game_loop");
    while game_running {
        game_loop(&mut game_running, &mut bodies);
        next_frame().await;
        continue;
    }
}

fn app_setup() -> Conf {
    info!("Beggining app_setup");
    Conf {
        window_height: 1440,
        window_width: 2560,
        window_title: "Stellarus".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

fn game_loop(game_running:&mut bool, bodies: &mut Vec<Body>) {
    clear_background(BLACK);
    let fps = macroquad::time::get_fps();
    draw_text(&fps.to_string(), 100.0, 100.0, 100.0, WHITE);

    move_bodies(bodies);
    draw_bodies(bodies);
    attract_bodies(bodies);

    if macroquad::input::is_key_down(KeyCode::Escape) {
        debug!("Keydown Esc");
        *game_running = false;       
    }    
}

fn draw_bodies(bodies: &Vec<Body>) {
    for b in bodies {
        draw_circle(b.position.x, b.position.y, ((b.mass as f32)/3.0).sqrt(), WHITE);
    }
}

fn attract_bodies(bodies: &mut Vec<Body>) {
    let bod2 = bodies.clone();
    for b in bodies {
        for b2 in &bod2 {
            b.apply_gravity(&b2);
        }
    }
}

fn move_bodies(bodies: &mut Vec<Body>) {
    for b in bodies {
        let v = b.velocity;
        b.position += v;
    }
}

// fn merge_bodies(bodies: &mut Vec<Body>) {
//     for b in bodies {
        
//     }
// }
