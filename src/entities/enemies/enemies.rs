//use crate::entities::DrawableEntity;
//use crate::entities::CollideEntity;
use crate::entities::player::player::Player;
use std::time::Duration;
use ggez::*;
use crate::entities::enemies::ai::AI;

pub trait Enemy {//: CollideEntity + DrawableEntity {
    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player);
    fn take_dmg(&mut self, ctx: &mut Context, dmg_to_take: f32);
    fn get_ai(&self) -> AI;
    //fn get_hitbox(&self) -> graphics::Rect;
    fn draw(&self, ctx: &mut Context) -> GameResult;
}

pub struct Enemies {
    enemies: Vec<Box<dyn Enemy>>,
}

impl Enemies {
    pub fn new(ctx: &mut Context, enemies_vec: Box<dyn Enemy>) -> Enemies {
        let mut e = Vec::new();
        e.push(enemies_vec);
        Enemies {
            enemies: e,
        }
    }
}

//impl DrawableEntity for Enemies {
//    fn draw(&self, ctx: &mut Context) -> GameResult {
//        
//        Ok(())
//    }
//}
//
//impl CollideEntity for Enemies {
//    fn get_hitbox(&self) -> graphics::Rect {
//		let img = graphics::Image::new(ctx, "/blob.png").unwrap();
//        img.dimensions()
//    }
//}

impl Enemy for Enemies {
    fn take_dmg(&mut self, ctx: &mut Context, dmg_to_take: f32)	{
        
    }

    fn get_ai(&self) -> AI {
        AI {
        
        }
    }

    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player) {
        for me in &mut self.enemies {
            me.update(ctx, delta, player);
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        for me in &self.enemies {
            me.draw(ctx)?;
        }
        Ok(())
    }
}