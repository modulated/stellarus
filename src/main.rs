use macroquad::prelude::*;
use ::rand::thread_rng;

use crate::body::Body;
mod body;

#[macroquad::main(app_setup)]
async fn main() {    
    let mut game_running = true;
    let mut bodies = Vec::new();
    let mut rng = thread_rng();

    for _ in 1..1000 {
        bodies.push(Body::random(&mut rng, 100));
    }

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
        window_height: 1800,
        window_width: 2880,
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

fn attract_bodies(bodies: &Vec<Body>) {
    for b in bodies {

    }
}

fn move_bodies(bodies: &mut Vec<Body>) {
    for b in bodies {
        let v = b.velocity;
        b.position += v;
    }
}

fn get_influence_score(bodies: &Vec<Body>, body: &Body, size: usize) -> Vec<(usize, f32)> {
    let mut t = bodies.iter().enumerate().map(|x| {
        if x.1 == body {
            return (x.0,0.0);
        } 
        return (x.0, x.1.mass as f32/((x.1.position - body.position).length()).abs());
    }).collect::<Vec<(usize, f32)>>();
    t.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
    t.truncate(size);
    t.split_at(size).1.to_owned()
}

// fn get_bodies_influence(bodies: &mut Vec<Body>, size: usize) -> Vec<Vec<Body>> {
//     let mut res: Vec<Vec<Body>> = Vec::new();
//     for b in bodies {
//         get_influence_score(bodies, &bodies[0], size);
//     }
//     res
// }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {        
        let mut bodies = Vec::new();
        let mut rng = thread_rng();
    
        for _ in 1..100 {
            bodies.push(Body::random(&mut rng, 100));
        }

        let r = get_influence_score(&bodies, &bodies[0], 4);
        println!("{:?}", r);
        assert_eq!(r.len(), 100);
    }
}