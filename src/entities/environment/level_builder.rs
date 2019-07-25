use ggez::{Context, error::GameError, GameResult};
use ggez::graphics::{Image, WHITE};

use super::level::Level;
use super::wall::Wall;

const WALL_WIDTH: usize = 100;
const WALL_HEIGHT: usize = 100;

pub struct LevelBuilder {
    default: Image,
    floor: Option<Image>,
    wall: Option<Image>,
    tile_width: usize,
    tile_height: usize,
}

impl LevelBuilder {
    pub fn new(ctx: &mut Context, default: Option<&Image>) -> LevelBuilder {
        let default = match default {
            Some(img) => img.clone(),
            None => Image::solid(ctx, 64u16, WHITE).unwrap(),
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

    fn validate_image_size(&self, image: &Image) -> GameResult {
        let image_width = image.width() as usize;
        let image_height = image.height() as usize;
        if image_width == self.tile_width && image_height == self.tile_height {
                return Err(GameError::ResourceLoadError(
                format!(
                    "Tile dimension mismatch: received ({},{}) expected ({},{})",
                    image_width, image_height,
                    self.tile_width, self.tile_height,
                )
            ))
        } else {
            Ok(())
        }
    }

    pub fn set_floor_image(&mut self, image: &Image) -> GameResult {
        self.validate_image_size(image)?;
        self.floor = Some(image.clone());
        Ok(())
    }

    pub fn set_wall_image(&mut self, image: &Image) -> GameResult {
        self.validate_image_size(image)?;
        self.wall = Some(image.clone());
        Ok(())
    }

    //pub from_level_file() -> ???;

    pub fn sample1(ctx: &mut Context) -> Level {
        let w = vec![
            (350.0, 150.0),
            (350.0, 250.0),
            (350.0, 350.0),
        ];
        LevelBuilder::generate_level(ctx, w)
    }

    pub fn sample2(ctx: &mut Context) -> Level {
        let mut w = Vec::new();
        // top bottom
        for x in (0..800).step_by(WALL_WIDTH) {
            for y in &[0, (600-WALL_HEIGHT)] {
                w.push((x as f32, *y as f32));
                //println!("{},{}", x, y);
            }
        }
        // left right
        for x in &[0, (800-WALL_WIDTH)] {
            for y in (WALL_HEIGHT..(600-WALL_HEIGHT)).step_by(WALL_HEIGHT) {
                w.push((*x as f32, y as f32));
                //println!("{},{}", x, y);
            }
        }
        LevelBuilder::generate_level(ctx, w)
    }

    fn generate_level(ctx: &mut Context, points: Vec<(f32, f32)>) -> Level {
        let walls = points.iter().map(|p| Wall::new(ctx, p.0, p.1)).collect();
        Level::new(walls)
    }
}