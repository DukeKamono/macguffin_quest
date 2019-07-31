use crate::entities::player::player::Player;
use crate::entities::enemies::enemies::Enemy;
use crate::entities::enemies::ai::AI;
use ggez::nalgebra as na;
use ggez::*;
use std::time::Duration;
//use rand::prelude::*;

use super::super::{CollideEntity, DrawableEntity};
use crate::ui::DmgText;

pub struct Skeleton {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub sprite: graphics::Image,
    pub hitbox: graphics::Rect,
    dmg_text: Vec<DmgText>,
}

impl Skeleton {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32) -> Skeleton {
        let img = graphics::Image::new(ctx, "/blob.png").unwrap();
        let hb = img.dimensions();
        let dmg_text = Vec::new();

        Skeleton {
            x: xpos,
            y: ypos,
            hp: 20.0,
            atk: 3.0,
            def: 1.0,
            sprite: img,
            hitbox: hb,
            dmg_text,
        }
    }

    pub fn take_dmg(&mut self, ctx: &mut Context, dmg_to_take: f32) {
        self.hp -= dmg_to_take;
        self.dmg_text.push(DmgText::new(ctx, self.x, self.y, dmg_to_take));
        // Check for death and maybe call a death function.
    }

    pub fn update(&mut self, delta: Duration) {
        self.dmg_text.retain(|t| t.live());
        self.dmg_text.iter_mut().for_each(|t| t.update(delta));
    }
}

impl DrawableEntity for Skeleton {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, &self.sprite, dp)?;

        self.dmg_text.iter().for_each(|t| t.draw(ctx));

        Ok(())
    }
}

impl CollideEntity for Skeleton {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.hitbox;
        r.x = self.x;
        r.y = self.y;
        r
    }
}

impl Enemy for Skeleton {
    fn get_ai(&self) -> AI {
        AI {
        
        }
    }

    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player) {
        self.update(delta);
        
        if self.collision(player) {
            player.take_dmg(self.atk);
        }
        
        if let Some(atk) = &player.atk_box {
            if self.collision(atk) {
                self.take_dmg(ctx, player.atk);
            }
        }
    }
}