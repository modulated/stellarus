use macroquad::prelude::*;
use rand::gen_range;

#[derive(Debug, PartialEq, Clone)]
pub struct Body {
	pub position: Vec2,
	pub velocity: Vec2,
	pub mass: u32
}

impl Body {
	pub fn random(max_mass: u32) -> Body {
		Body {
			position: Vec2::new(gen_range(0.0, screen_width()), gen_range(0.0, screen_height())),
			velocity: Vec2::new(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0)),						
			mass: gen_range(10, max_mass)
		}
	}

	pub fn apply_gravity(&mut self, body: &Body) {
		let diff = self.position - body.position;
		if diff.length() < 0.01 {
			return;
		}
		let f = (body.mass as f32)/diff.length();
		let a = (diff * f * 0.00001)/self.mass as f32;
		self.velocity -= a;

		// debug!("Diff: {}", diff);
		// debug!("Force: {}", f);
		// debug!("Accel: {}", a);
	}
}