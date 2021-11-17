extern crate piston_window;
use piston_window::*;

struct Ball {
	x: f64,
	y: f64,
	r: f64,
	c: f64,
	s: f64,
	angle: f64,
	speed: f64,
}

struct Player {
	y: f64,
	key_move: f64,
}

fn main() {
	let window_x = 640;
	let window_y = 480;
	
	let pi = 3.14;
	let two_pi = 2.0*pi;

	let mut window: PistonWindow = WindowSettings::new("Pong!", (window_x, window_y))
		.exit_on_esc(true)
		.build()
		.unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

	let mut player1 = Player {
		y: 0.0,
		key_move: 0.0,
	};

	let mut player2 = Player {
		y: 0.0,
		key_move: 0.0,
	};
	
	let mut ball = Ball {
		x: 400.0,
		y: 100.0,
		r: 40.0,
		c: 0.0,
		s: 0.0,
		angle: pi*1.2,
		speed: 0.3,
	};

	let player_speed = 1.0;
	let player_width = 20.0;
	let player_height = 50.0;
	let player_space = 20.0;
	
	ball.c = ball.angle.cos();
	ball.s = ball.angle.sin();
	
	// print!("{}", angle.sin());
	while let Some(e) = window.next() {
		window.draw_2d(&e, |c, g, _d| {
			clear([0.1, 0.1, 0.1, 1.0], g);

			// Player 1
			rectangle([1.0, 0.0, 0.0, 1.0], [player_space, player1.y, player_width, player_height], c.transform, g);

			// Player 2
			rectangle([1.0, 0.0, 0.0, 1.0], [window_x as f64 - (player_width + player_space), player2.y, player_width, player_height], c.transform, g);

			// Ball
			ellipse([0.9, 0.9, 0.9, 1.0], [ball.x, ball.y, ball.r, ball.r], c.transform, g);
		});

		if !e.press_args().is_none() {
			let k = e.press_args().unwrap();

			match k {
				Button::Keyboard(Key::W) => player1.key_move = -1.0,
				Button::Keyboard(Key::S) => player1.key_move = 1.0,

				Button::Keyboard(Key::O) => player2.key_move = -1.0,
				Button::Keyboard(Key::L) => player2.key_move = 1.0,
				_ => (),
			}
			
			println!("Key pressed: {:?}", k);
		}

		if !e.release_args().is_none() {
			let k = e.release_args().unwrap();

			match k {
				Button::Keyboard(Key::W) => player1.key_move = 0.0,
				Button::Keyboard(Key::S) => player1.key_move = 0.0,

				Button::Keyboard(Key::O) => player2.key_move = 0.0,
				Button::Keyboard(Key::L) => player2.key_move = 0.0,
				_ => (),
			}
			
			println!("Key released: {:?}", k);
		}

		println!("Player 1: {}", player1.key_move);

		player1.y += player1.key_move * player_speed;
		player2.y += player2.key_move * player_speed;
		
		ball.x += ball.c * ball.speed;
		ball.y += ball.s * ball.speed;

		let mut collided = false;
		println!("ball y: {}", ball.y);

		// Player 1
		if (ball.x <= player_space + player_width) {
			if (player1.y <= ball.y && ball.y <= (player1.y + player_height)) || (player1.y <= (ball.y + ball.r) && (ball.y + ball.r) <= (player1.y + player_height)) {
				ball.angle = pi - ball.angle;
				collided = true;
				ball.speed += 0.2;
			}
		}

		// Player 2
		if (ball.x >= (window_x as f64) - player_space - player_width - ball.r) {
			if (player2.y <= ball.y && ball.y <= (player2.y + player_height)) || (player2.y <= (ball.y + ball.r) && (ball.y + ball.r) <= (player2.y + player_height)) {
				ball.angle = pi - ball.angle;
				collided = true;
				ball.speed += 0.2;
			}
		}

		// Top
		if ball.y <= 0.0 {
			ball.angle = two_pi - ball.angle;
			collided = true;
		}

		// Bottom
		if ball.y + ball.r >= window_y.into() {
			ball.angle = two_pi - ball.angle;
			collided = true;
		}

		// Right
		if ball.x + ball.r >= window_x.into() {
			println!("Player 1 Won!");
		}

		// Left
		if ball.x <= 0.0 {
			println!("Player 2 Won!");
		}

		// Calculate new values
		if collided {
			println!("Calculating new values...");
			ball.c = ball.angle.cos();
			ball.s = ball.angle.sin();
		}
	}
}
