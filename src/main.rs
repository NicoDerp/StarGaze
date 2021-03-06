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
	score: u8,
}

fn main() {
	// 640, 480
	let window_x = 1300;
	let window_y = 800;
	
	let pi = 3.14;
	let two_pi = 2.0*pi;

	let mut window: PistonWindow = WindowSettings::new("Pong!", (window_x, window_y))
		.exit_on_esc(true)
		.vsync(true)
		.build()
		.unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

	let mut player1 = Player {
		y: 0.0,
		key_move: 0.0,
		score: 0,
	};

	let mut player2 = Player {
		y: 0.0,
		key_move: 0.0,
		score: 0,
	};
	
	let mut ball = Ball {
		x: (window_x/2).into(),
		y: (window_y/2).into(),
		r: 40.0,
		c: 0.0,
		s: 0.0,
		angle: pi*1.3,
		speed: 0.3,
	};

	let player_speed = 0.8;
	let player_width = 10.0;
	let player_height = 80.0;
	let player_space = 20.0;
	
	ball.c = ball.angle.cos();
	ball.s = ball.angle.sin();

	let font = include_bytes!("./Fonts/bit5x3.ttf");
	
	let mut glyphs = Glyphs::from_bytes(
		font,
		window.create_texture_context(),
		TextureSettings::new(),
	).unwrap();
	
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

			let transform = c.transform.trans((window_x/2 - 120).into(), 80.0);
			let _ = text::Text::new_color([0.9, 0.9, 0.9, 1.0], 50).draw(
				format!("{} | {}", player1.score, player2.score).as_str(),
				&mut glyphs,
				&c.draw_state,
				transform, g
			);
			glyphs.factory.encoder.flush(_d);
			// println!("{:?}", ooga);
		});

		// println!("{} | {}", player1.score, player2.score);

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

		player1.y += player1.key_move * player_speed;
		player2.y += player2.key_move * player_speed;
		
		if player1.y <= 0.0 { player1.y = 0.0; }
		if player1.y + player_height >= window_y.into() { player1.y = window_y.into(); player1.y -= player_height; }
		if player2.y <= 0.0 { player2.y = 0.0; }
		if player2.y + player_height >= window_y.into() { player2.y = window_y.into(); player2.y -= player_height; }
				
		ball.x += ball.c * ball.speed;
		ball.y += ball.s * ball.speed;

		let mut collided = false;
		// println!("ball y: {}", ball.y);

		// Player 1
		if ball.x <= player_space + player_width {
			if (player1.y <= ball.y && ball.y <= (player1.y + player_height)) || (player1.y <= (ball.y + ball.r) && (ball.y + ball.r) <= (player1.y + player_height)) {
				ball.angle = pi - ball.angle;
				collided = true;
				ball.speed += 0.2;
			}
		}

		// Player 2
		if ball.x >= (window_x as f64) - player_space - player_width - ball.r {
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
			player1.score += 1;
			ball.speed = 0.5;
			ball.x = (window_x / 2).into();
			ball.y = (window_y / 2).into();
		}

		// Left
		if ball.x <= 0.0 {
			player2.score += 1;
			ball.speed = 0.5;
			ball.x = (window_x / 2).into();
			ball.y = (window_y / 2).into();
		}

		// Calculate new values
		if collided {
			println!("Calculating new values...");
			ball.c = ball.angle.cos();
			ball.s = ball.angle.sin();
		}
	}
}
