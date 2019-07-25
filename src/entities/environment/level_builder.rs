use ggez::{Context, error::GameError, GameResult};
use ggez::graphics::{Image, Rect, WHITE};

use crate::sprites::Sprite;
use super::level::Level;
use super::tile::Tile;

pub struct LevelBuilder {
    default: Sprite,
    floor: Option<Sprite>,
    wall: Option<Sprite>,
    tile_width: usize,
    tile_height: usize,
}

impl LevelBuilder {
    pub fn new(ctx: &mut Context, default: Option<&Sprite>) -> LevelBuilder {
        let default = match default {
            Some(img) => img.clone(),
            None => Sprite::new(&Image::solid(ctx, 64u16, WHITE).unwrap(), Rect::new(0f32,0f32,64f32,64f32)).unwrap(),
        };
        let tile_width = default.width() as usize;
        let tile_height = default.height() as usize;
        LevelBuilder {
            default,
            floor: None,
            wall: None,
            tile_width,
            tile_height,
        }
    }

    fn validate_image_size(&self, image: &Sprite) -> GameResult {
        let image_width = image.width() as usize;
        let image_height = image.height() as usize;
        if image_width == self.tile_width && image_height == self.tile_height {
            Ok(())
        } else {
            return Err(GameError::ResourceLoadError(
                format!(
                    "Tile dimension mismatch: received ({},{}) expected ({},{})",
                    image_width, image_height,
                    self.tile_width, self.tile_height,
                )
            ))
        }
    }

    pub fn set_floor_image(&mut self, image: &Sprite) -> GameResult {
        self.validate_image_size(image)?;
        self.floor = Some(image.clone());
        Ok(())
    }

    pub fn set_wall_image(&mut self, image: &Sprite) -> GameResult {
        self.validate_image_size(image)?;
        self.wall = Some(image.clone());
        Ok(())
    }

    //pub from_level_file() -> ???;

    pub fn sample1(&self) -> Level {
        let w = vec![
            (350.0, 150.0),
            (350.0, 250.0),
            (350.0, 350.0),
        ];
        self.generate_level(w)
    }

    pub fn sample2(&self) -> Level {
        let mut w = Vec::new();
        // top bottom
        for x in (0..800).step_by(self.tile_width) {
            for y in &[0, (600-self.tile_height)] {
                w.push((x as f32, *y as f32));
                //println!("{},{}", x, y);
            }
        }
        // left right
        for x in &[0, (800-self.tile_width)] {
            for y in (self.tile_height..(600-self.tile_height)).step_by(self.tile_height) {
                w.push((*x as f32, y as f32));
                //println!("{},{}", x, y);
            }
        }
        self.generate_level(w)
    }

    fn generate_level(&self, points: Vec<(f32, f32)>) -> Level {
        let wall_image = if let Some(image) = &self.wall {
            image
        } else {
            &self.default
        };
        let tiles = points.iter().map(|p| Tile::new(&wall_image, p.0, p.1)).collect();
        Level::new(tiles)
    }
}