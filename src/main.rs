extern crate piston_window;
use piston_window::*;
fn main() {
	let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480)).exit_on_esc(true).build().unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

	let mut x = 0.0;
	let mut y = 0.0;
	while let Some(e) = window.next() {
		window.draw_2d(&e, |c, g, _d| {
			clear([0.5, 1.0, 0.5, 1.0], g);
			
			rectangle([1.0, 0.0, 0.0, 1.0], [x, y, 50.0, 50.0], c.transform, g);
		});
		x+=0.2;
		y+=0.2;
	}
}

// https://crates.io/crates/piston

