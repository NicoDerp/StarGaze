extern crate piston_window;
use piston_window::*;

struct Ball {
	x: f64,
	y: f64,
	c: f64,
	s: f64,
	angle: f64,
	speed: f64,
}

fn main() {
	let window_x = 640;
	let window_y = 480;
	let pi = 3.14;
	let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (window_x, window_y)).exit_on_esc(true).build().unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

	let mut player1_y = 0.0;
	let mut player2_y = 0.0;
	
	let mut ball = Ball {
		x: 100.0,
		y: 100.0,
		c: 0.0,
		s: 0.0,
		angle: pi/3.0,
		speed: 1.0,
	};

	ball.c = ball.angle.cos();
	ball.s = ball.angle.sin();
	
	// print!("{}", angle.sin());
	while let Some(e) = window.next() {
		window.draw_2d(&e, |c, g, _d| {
			clear([0.1, 0.1, 0.1, 1.0], g);

			// Player 1
			rectangle([1.0, 0.0, 0.0, 1.0], [20.0, player1_y, 20.0, 50.0], c.transform, g);

			// Player 2
			rectangle([1.0, 0.0, 0.0, 1.0], [window_x as f64 - (20.0 + 20.0), player2_y, 20.0, 50.0], c.transform, g);

			// Ball
			ellipse([0.9, 0.9, 0.9, 1.0], [ball.x, ball.y, 40.0, 40.0], c.transform, g);
		});

		println!("had: {:?}", e.press_args());

		if let Some(Button::Keyboard(key)) = e.press_args() {
			if key == Key::W {
				println!("Pressed keyboard key '{:?}'", key);
			}
		}

		//let b = Input::Button(ButtonArgs{
		//	state: ButtonState::Press,
		//	button: Button::Keyboard(Key::W), // key
		//	scancode: Some(11),
		//});

		//let b = Input::Text(String::from("W"));
		//let mut evt = Event::Input(b, Some(0));
		// let mut evt = Event::Input(Input::Text(String::from("W")), Some(0));

		//let a = ButtonEvent::button(&evt, keyPressed);
		//let mut a = ButtonEvent::button_args(&evt);
		//if (!a.is_none()) {
		//	print!("aasdasd");
		//	print!("{}", a.unwrap().scancode.unwrap());
		//}
		
		ball.x += ball.c * ball.speed;
		ball.y += ball.s * ball.speed;
	}
}

fn keyPressed(args: ButtonArgs) {
	print!("Key state: {}", args.state == ButtonState::Release);
}

// https://crates.io/crates/piston

