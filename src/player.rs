//use ggez::{graphics, nalgebra as na};
//use ggez::input::keyboard;
use ggez::*;

pub struct Player {
	pub x: f32,
	pub y: f32,
	//sprite: graphics::Image,
	//hitbox: graphics::Image,
}

impl Player {
	pub fn new(_ctx: &mut Context) -> Player {
		Player {
			x: 10.0,
			y: 10.0,
			// Trying to do this https://docs.rs/ggez/0.5.0-rc.2/ggez/filesystem/index.html
			//sprite: graphics::Image::new(ctx, "/texture/pong_spritesheet.png").unwrap(),
			//hitbox: graphics::Image::new(ctx, "/assets/pong_spritesheet.png").unwrap(),
		}
	}

	//pub fn draw(ctx: &mut Context) -> std::result::Result<ggez::graphics::Mesh, ggez::error::GameError> {
	//	// Create a circle at `position_x` and draw
	//	let circle = graphics::Mesh::new_circle(
	//		ctx,
	//		graphics::DrawMode::fill(),
	//		na::Point2::new(0.0, 380.0),
	//		100.0,
	//		2.0,
	//		graphics::WHITE,
	//	);
	//	circle
	//}
}