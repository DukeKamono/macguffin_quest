use ggez::nalgebra as na;
//use ggez::input::keyboard;
use ggez::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
	pub hp: f32,
    pub sprite: graphics::Image,
    //hitbox: graphics::Image,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Player {
        Player {
            x: 10.0,
            y: 10.0,
			hp: 30.0,
            sprite: graphics::Image::new(ctx, "/pong_spritesheet.png").unwrap(),
            //hitbox: graphics::Image::new(ctx, "/assets/pong_spritesheet.png").unwrap(),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        // This sets the location of the thing going to be drawn. (player)
        let draw_param = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        // This draws the player.
        graphics::draw(ctx, &self.sprite, draw_param).expect("Can't display Player!");
    }
}
