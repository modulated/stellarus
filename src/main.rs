use macroquad::prelude::*;
mod body;
use body::Body;

#[derive(Default)]
struct GameState {
    bodies: Vec<Body>,
    game_running: bool,
    sim_running: bool,
}

#[macroquad::main(app_setup)]
async fn main() {    
    let mut state = GameState {
        game_running: true,
        ..Default::default()
    };

    // for _ in 1..300 {
    //     state.bodies.push(Body::random(100));
    // }

    state.bodies.push(Body {
        mass: 100000,
        velocity: Vec2::new(0.0, 0.0),
        position: Vec2::new (screen_width()/2.0, screen_height()/2.0),
        color: YELLOW
    });

    state.bodies.push(Body {
        mass: 2000,
        velocity: Vec2::new(2.0, 0.0),
        position: Vec2::new (screen_width()/2.0, 230.0),
        color: BLUE
    });

    state.bodies.push(Body {
        mass: 2000,
        velocity: Vec2::new(2.3, 0.0),
        position: Vec2::new (screen_width()/2.0, 180.0),
        color: PINK
    });

    info!("Beggining game_loop");
    while state.game_running {
        game_loop(&mut state);
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

fn game_loop(state: &mut GameState) {
    clear_background(BLACK);
    let fps = macroquad::time::get_fps();
    draw_text(&format!("FPS: {}",&fps.to_string()), 100.0, 60.0, 50.0, WHITE);
    draw_text(&format!("Bodies: {}",&state.bodies.len().to_string()), 100.0, 100.0, 50.0, WHITE);

    if state.sim_running {
        move_bodies(&mut state.bodies);
        // collide_and_merge_bodies(&mut state.bodies);
        attract_bodies(&mut state.bodies);
    }    
    
    draw_bodies(&mut state.bodies);
    handle_input(state);
}

fn handle_input(state: &mut GameState) {
    if macroquad::input::is_key_down(KeyCode::Escape) {
        debug!("Keydown Esc");
        state.game_running = false;       
    }  

    if macroquad::input::is_key_pressed(KeyCode::Space) {
        debug!("KeyPressed Space");
        state.sim_running = !state.sim_running;
    }
}

fn draw_bodies(bodies: &Vec<Body>) {
    for b in bodies {
        draw_circle(b.position.x, b.position.y, ((b.mass as f32)/3.0).sqrt(), b.color);
    }
}

fn attract_bodies(bodies: &mut Vec<Body>) {
    let bod2 = bodies.clone();
    for (i, b) in bodies.iter_mut().enumerate() {
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

fn collide_and_merge_bodies(bodies: &mut Vec<Body>) {
    
    let immut_bodies = &*bodies.clone();
    for (i, b1) in immut_bodies.iter().enumerate() {
        for (j, b2) in immut_bodies[i..].iter().enumerate() {

            let dist = ((b1.mass as f32)/3.0).sqrt() + ((b2.mass as f32)/3.0).sqrt();
            if (b1.position - b2.position).length() < dist {
                // merge_bodies(bodies, i, j);
                return;
            }
        }
    }
}

fn merge_bodies(bodies: &mut Vec<Body>, index_a: usize, index_b: usize) {

    if bodies[index_a].mass < bodies[index_b].mass {
        let momentum = (bodies[index_a].velocity * bodies[index_a].mass as f32)/bodies[index_b].mass as f32;
        bodies[index_b].velocity += momentum;
        bodies[index_b].mass += bodies[index_a].mass;
        bodies.remove(index_a);
    } else {
        let momentum = (bodies[index_b].velocity * bodies[index_b].mass as f32)/bodies[index_a].mass as f32;
        bodies[index_a].velocity += momentum;
        bodies[index_a].mass += bodies[index_b].mass;
        bodies.remove(index_b);
    }
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_merge() {
        let mut bodies = vec![
            Body {
                mass: 50,
                position: Vec2::new(1.0, 1.0),
                velocity: Vec2::new(-1.0, 0.0)
            },
            Body {
                mass: 50,
                position: Vec2::new(1.0, 1.0),
                velocity: Vec2::new(1.0, 0.0)
            },
            Body {
                mass: 40,
                position: Vec2::new(100.0, 1.0),
                velocity: Vec2::new(0.0, 0.0)
            }
        ];

        merge_bodies(&mut bodies);

        assert_eq!(bodies.len(), 2);
        assert_eq!(bodies[0].mass, 100);
        assert_eq!(bodies[0].velocity.x, 0.0);
    }
}

