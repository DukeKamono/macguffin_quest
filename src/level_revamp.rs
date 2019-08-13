use std::collections::{HashMap, VecDeque};
use std::time::Duration;

use ggez::{Context, GameResult};
use ggez::graphics::{Color, DrawParam, Image, self};
use ggez::timer;

use crate::entity_revamp::{Entity, EntityBuilder};
use crate::sprite_revamp::{Sprite, SpriteBuilder};

#[derive(Clone)]
struct Floor {
    sprite: Sprite,
    param: DrawParam,
}

impl Floor {
    fn new(sprite: &Sprite, param: DrawParam) -> Self {
        Floor { sprite: sprite.clone(), param }
    }

    fn update(&mut self, delta: Duration) {
        self.sprite.update(delta);
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, self.param)
    }
}




#[derive(Clone)]
pub struct Level {
    floor: Vec<Floor>,
    walls: Vec<Entity>,
    enemies: VecDeque<Entity>,
    // items
}

impl Level {
    fn new(floor: Vec<Floor>, walls: Vec<Entity>, enemies: VecDeque<Entity>) -> Self {
        Level {
            floor,
            walls,
            enemies,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, player: &mut Entity) {
        let delta = timer::delta(ctx);

        self.floor.iter_mut().for_each(|f| f.update(delta));

        for w in &mut self.walls {
            w.update(ctx, player, &mut self.enemies);
        }

        for _ in 0usize..self.enemies.len() {
            let mut enemy = self.enemies.pop_front().unwrap();
            enemy.update(ctx, player, &mut self.enemies);
            self.enemies.push_back(enemy);
        }
    }

    pub fn draw(&self, ctx: &mut Context, showhitbox: bool) -> GameResult {
        for f in &self.floor {
            f.draw(ctx)?;
        }

        for w in &self.walls {
            w.draw(ctx, showhitbox)?;
        }

        for e in &self.enemies {
            e.draw(ctx, showhitbox)?;
        }

        Ok(())
    }

    pub fn get_enemies(&mut self) -> &mut VecDeque<Entity> {
        &mut self.enemies
    }
}




type LocationAndType = (i64, i64, usize);

pub struct LevelBuilder {
    floor_default: Sprite,
    floor_sprites: HashMap<usize, Sprite>,
    wall_default: Sprite,
    wall_sprites: HashMap<usize, Sprite>,
    floor: Vec<LocationAndType>,
    walls: Vec<LocationAndType>,
    enemies: Vec<LocationAndType>,
}

impl LevelBuilder {
    pub fn new(ctx: &mut Context) -> Self {
        let default_img = Image::solid(ctx, 64u16, [0.38f32, 0.21f32, 0.19f32, 1f32].into()).unwrap();
        let floor_default = SpriteBuilder::new(&default_img).add_frame(default_img.dimensions(), None, None, None).build_sprite().unwrap();
        let floor_sprites = HashMap::new();
        
        let default_img = Image::solid(ctx, 64u16, [0.19f32, 0.21f32, 0.38f32, 1f32].into()).unwrap();
        let wall_default = SpriteBuilder::new(&default_img).add_frame(default_img.dimensions(), None, None, None).build_sprite().unwrap();
        let wall_sprites = HashMap::new();
        
        let floor = Vec::new();
        let walls = Vec::new();
        let enemies = Vec::new();

        LevelBuilder {
            floor_default,
            floor_sprites,
            wall_default,
            wall_sprites,
            floor,
            walls,
            enemies,
        }
    }

    pub fn add_floor_sprite(mut self, key: usize, value: &Sprite) -> Self {
        self.floor_sprites.insert(key, value.clone());
        self
    }

    pub fn add_wall_sprite(mut self, key: usize, value: &Sprite) -> Self {
        self.wall_sprites.insert(key, value.clone());
        self
    }

    pub fn add_floors(mut self, floor: &Vec<LocationAndType>) -> Self {
        self.floor.extend(floor);
        self
    }

    pub fn add_walls(mut self, walls: &Vec<LocationAndType>) -> Self {
        self.walls.extend(walls);
        self
    }

    pub fn add_enemies(mut self, enemies: &Vec<LocationAndType>) -> Self {
        self.enemies.extend(enemies);
        self
    }

    pub fn from_file(mut self, ctx: &mut Context, path: String) -> Self {
        unimplemented!("Loading level from file not implemented yet");
        self
    }

    pub fn sample(mut self, ctx: &mut Context) -> Self {
        let floor = Image::solid(ctx, 64u16, Color::new(0f32, 1f32, 0f32, 1f32)).unwrap();
        let floor = SpriteBuilder::new(&floor).add_frame(floor.dimensions(), None, None, None).build_sprite().unwrap();
        self = self.add_floor_sprite(0usize, &floor);
        
        let wall = Image::solid(ctx, 64u16, Color::new(1f32, 0f32, 0f32, 1f32)).unwrap();
        let wall = SpriteBuilder::new(&wall).add_frame(wall.dimensions(), None, None, None).build_sprite().unwrap();
        self = self.add_floor_sprite(0usize, &wall);

        self = self.add_floors(&vec![
            (336i64, 172i64, 0usize), // left side
            (336i64, 236i64, 0usize),
            (336i64, 300i64, 0usize),
            (336i64, 364i64, 0usize),
            (336i64, 428i64, 0usize),
            (400i64, 172i64, 0usize), // top
            (464i64, 172i64, 0usize), // right side
            (464i64, 236i64, 0usize),
            (464i64, 300i64, 0usize),
            (464i64, 364i64, 0usize),
            (464i64, 428i64, 0usize),
            (400i64, 428i64, 0usize), // bottom
        ]);

        self = self.add_walls(&vec![
            (400i64, 236i64, 0usize),
            (400i64, 300i64, 0usize),
            (400i64, 364i64, 0usize),
        ]);

        self = self.add_enemies(&vec![
            (528i64, 236i64, 0usize),
            (528i64, 364i64, 0usize),
        ]);

        self
    }

    pub fn build_level(mut self, ctx: &mut Context) -> GameResult<Level> {
        let mut floor = Vec::new();
        for f in self.floor {
            let sprite = match self.floor_sprites.get(&f.2) {
                Some(s) => s,
                None => &self.floor_default,
            };
            let dp = DrawParam::default().dest([f.0 as f32, f.1 as f32]);
            floor.push(Floor::new(sprite, dp));
        }

        let mut walls = Vec::new();
        for w in self.walls {
            let sprite = match self.wall_sprites.get(&w.2) {
                Some(s) => s,
                None => &self.wall_default,
            };
            walls.push(EntityBuilder::build_wall(&sprite, [w.0 as f32, w.1 as f32])?);
        }

        let mut enemies = VecDeque::new();
        for e in self.enemies {
            let enemy = match e.2 {
                _ => EntityBuilder::build_enemy(ctx, [e.0 as f32, e.1 as f32])?,
            };
            enemies.push_back(enemy);
        }

        Ok(Level::new(floor, walls, enemies))
    }
}