use super::glam::Vec2;
use rand::prelude::*;
use macroquad::prelude::{screen_height, screen_width};

#[derive(Debug, PartialEq)]
pub struct Body {
    pub mass: i32,    
    pub position: Vec2,
    pub velocity: Vec2
}

impl Body {
    #[allow(dead_code)]
    pub fn new(mass: i32, position: Vec2, velocity: Vec2) -> Body {
        Body {
            mass,
            position,
            velocity
        }
    }

    pub fn random(rng: &mut ThreadRng, max_mass: i32) -> Body {
        Body {
            mass: rng.gen_range(0..max_mass),
            position: Vec2::new(rng.gen_range(0.0..screen_width()), rng.gen_range(0.0..screen_height())),
            velocity: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
        }
    }
}

