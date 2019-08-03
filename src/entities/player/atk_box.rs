use ggez::*;

use super::super::{CollideEntity, DrawableEntity, Direction};

pub struct AtkBox {
    pub duration: f32,
    pub x: f32,
    pub y: f32,
    pub hitbox: graphics::Rect,
    pub shape: graphics::Mesh,
}

impl AtkBox {
    pub fn new(ctx: &mut Context, duration: f32, xpos: f32, ypos: f32, direction: & Direction) -> AtkBox {
		
		let mut xpos = xpos;
		let mut ypos = ypos;
		
		// The player is kinda off centered and these values need to be adjusted.
		match direction {
			Direction::Up => ypos -= 50.0,
			Direction::Down => ypos += 50.0,
			Direction::Left => xpos -= 50.0,
			Direction::Right => xpos += 50.0,
		}
		
        // radius of circle
        let r = 50f32;
        // create hit box
        let hb = graphics::Rect::new(0.0, 0.0, r * 2.0, r * 2.0);
        // create mesh
        let circle = graphics::MeshBuilder::new()
            .circle(
                graphics::DrawMode::fill(),
                nalgebra::Point2::new(r, r),
                r,
                1.0,
                graphics::WHITE,
            )
            .rectangle(graphics::DrawMode::stroke(1.0), hb, graphics::WHITE)
            .build(ctx)
            .unwrap();

        AtkBox {
            duration,
            x: xpos,
            y: ypos,
            hitbox: hb,
            shape: circle,
        }
    }
}

impl DrawableEntity for AtkBox {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(nalgebra::Point2::new(self.x, self.y));
        graphics::draw(ctx, &self.shape, dp)
    }
}

impl CollideEntity for AtkBox {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.hitbox;
        r.x = self.x;
        r.y = self.y;
        r
    }
}
