use std::collections::HashMap;

use ggez::graphics::{Image, Rect, WHITE};
use ggez::{error::GameError, Context, GameResult};

use super::level::Level;
use super::tile::Tile;
use crate::sprites::Sprite;

pub struct LevelBuilder {
    default: Sprite,
    tile_image: HashMap<usize, Sprite>,
    tile_width: usize,
    tile_height: usize,
}

impl LevelBuilder {
    pub fn new(ctx: &mut Context, default: Option<&Sprite>) -> LevelBuilder {
        let default = match default {
            Some(img) => img.clone(),
            None => Sprite::new(
                &Image::solid(ctx, 64u16, WHITE).unwrap(),
                Rect::new(0f32, 0f32, 64f32, 64f32),
            )
            .unwrap(),
        };
        let tile_image = HashMap::new();
        let tile_width = default.width() as usize;
        let tile_height = default.height() as usize;
        LevelBuilder {
            default,
            tile_image,
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
            Err(GameError::ResourceLoadError(format!(
                "Tile dimension mismatch: received ({},{}) expected ({},{})",
                image_width, image_height, self.tile_width, self.tile_height,
            )))
        }
    }

    pub fn set_tile_image(&mut self, key: usize, image: &Sprite) -> GameResult {
        self.validate_image_size(image)?;
        self.tile_image.insert(key, image.clone());
        Ok(())
    }

    //pub from_level_file() -> ???;

    pub fn _sample1(&self) -> Level {
        let w = vec![
            (350.0, 150.0, 1usize),
            (414.0, 150.0, 0usize),
            (350.0, 250.0, 1usize),
            (414.0, 250.0, 0usize),
            (350.0, 350.0, 1usize),
            (414.0, 350.0, 0usize),
        ];
        self.generate_level(w)
    }

    // attempts to frame an 800x600 window (may overlap with poor image dimensions)
    pub fn _sample2(&self) -> Level {
        let mut w = Vec::new();
        // top bottom
        for x in (0..800).step_by(self.tile_width) {
            for y in &[0, (600 - self.tile_height)] {
                w.push((x as f32, *y as f32, 1usize));
                //println!("{},{}", x, y);
            }
        }
        // left right
        for x in &[0, (800 - self.tile_width)] {
            for y in (self.tile_height..(600 - self.tile_height)).step_by(self.tile_height) {
                w.push((*x as f32, y as f32, 1usize));
                //println!("{},{}", x, y);
            }
        }
        self.generate_level(w)
    }

    pub fn sample3(&self) -> Level {
        let mut w = Vec::new();
        let tr = self.tile_width * 11usize;
        let br = self.tile_height * 8usize;
        let m = self.tile_width * 6usize;
        // corners
        w.push((0f32, 0f32, 5usize));
        w.push((tr as f32, 0f32, 5usize));
        w.push((0f32, br as f32, 5usize));
        w.push((tr as f32, br as f32, 5usize));
        // top and bottom
        for x in (self.tile_width..tr).step_by(self.tile_width) {
            w.push((x as f32, 0f32, 0usize));
            w.push((x as f32, br as f32, 1usize));
        }
        // left right
        for y in (self.tile_height..br).step_by(self.tile_height) {
            w.push((0f32, y as f32, 2usize));
            w.push((tr as f32, y as f32, 3usize));
        }
        // middle
        for y in (self.tile_height * 3usize..m).step_by(self.tile_height) {
            w.push((m as f32, y as f32, 4usize));
        }
        self.generate_level(w)
    }

    fn generate_level(&self, points: Vec<(f32, f32, usize)>) -> Level {
        let tiles = points
            .iter()
            .map(|p| {
                let image = match self.tile_image.get(&p.2) {
                    Some(image) => image,
                    None => &self.default,
                };
                Tile::new(&image, p.0, p.1)
            })
            .collect();
        Level::new(tiles)
    }
}
