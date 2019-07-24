use ggez::Context;

use super::level::Level;
use super::wall::Wall;

const WALL_WIDTH: usize = 100;
const WALL_HEIGHT: usize = 100;

pub struct LevelBuilder {
    // fields
}

impl LevelBuilder {
    /*
    pub fn new() -> LevelBuilder {
        LevelBuilder { }
    }
    */

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
            for y in (100..(600-WALL_HEIGHT)).step_by(WALL_HEIGHT) {
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