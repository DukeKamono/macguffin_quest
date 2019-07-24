use ggez::{Context, GameResult};
use ggez::graphics::Rect;
use super::super::{CollideEntity, DrawableEntity};
use super::wall::Wall;

pub struct Level {
    walls: Vec<Wall>,
}

impl Level {
    // should a new() really be provided?
    // instead for level to come from level_builder
    pub fn new(walls: Vec<Wall>) -> Level {
        Level{ walls }
    }
}

impl DrawableEntity for Level {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        for w in &self.walls {
            w.draw(ctx)?;
        }
        Ok(())
    }
}

impl CollideEntity for Level {
    fn get_hitbox(&self) -> Rect {
        let mut r = self.walls.first().unwrap().get_hitbox();
        for w in &self.walls {
            r = r.combine_with(w.get_hitbox());
        }
        r
    }

    fn get_sub_hitboxs(&self) -> Vec<Rect> {
        self.walls.iter().map(|w| w.get_hitbox()).collect()
    }
}