use super::super::{CollideEntity, DrawableEntity};
use super::tile::Tile;
use ggez::graphics::Rect;
use ggez::{Context, GameResult};

pub struct Level {
    tiles: Vec<Tile>,
}

impl Level {
    // should a new() really be provided?
    // instead for level to come from level_builder
    pub fn new(tiles: Vec<Tile>) -> Level {
        Level { tiles }
    }
}

impl DrawableEntity for Level {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        for w in &self.tiles {
            w.draw(ctx)?;
        }
        Ok(())
    }
}

impl CollideEntity for Level {
    fn get_hitbox(&self) -> Rect {
        let mut r = self.tiles.first().unwrap().get_hitbox();
        for w in &self.tiles {
            r = r.combine_with(w.get_hitbox());
        }
        r
    }

    fn get_sub_hitboxs(&self) -> Vec<Rect> {
        self.tiles.iter().map(|w| w.get_hitbox()).collect()
    }
}
