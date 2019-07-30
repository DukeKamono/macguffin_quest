

// contains all the information on entities
//mod entities;
// get collison trait from entities
use super::entities::{CollideEntity, DrawableEntity};
// get player struct to use
use super::entities::player::player::Player;
// get blob struct to use
use super::entities::enemies::blob::Blob;
// get wall struct to use
use super::entities::environment::{level::Level, level_builder::LevelBuilder};

//mod ui;
use super::ui::UI;

//mod sprites;
//use sprites::sprite::Sprite;
//use sprites::animated_sprite::*;
use super::sprites::*;

pub struct MainState {
    ui: UI,
    player: Player,
    blob: Vec<Blob>,
    level: Level,
    sprite: Sprite,
    animated: AnimatedSprite,
    rotation: f32,
}

impl CustomEventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> HandlerMessage {
        let playerx = self.player.x;
        let playery = self.player.y;

        self.player.update(ctx);

        if self.player.collision(&self.level) {
            self.player.move_location(playerx, playery);
        }

        for blob in &mut self.blob {
            if blob.collision(&self.player) {
                self.player.take_dmg(blob.atk);
            }

            if let Some(atk) = &self.player.atk_box {
                if blob.collision(atk) {
                    blob.take_dmg(self.player.atk);
                    self.ui.update_dmg_text(ctx, blob.x, blob.y, self.player.atk);
                }
            }
        }

        self.animated.animate(timer::delta(ctx));

        self.rotation += timer::duration_to_f64(timer::delta(ctx)) as f32;
        self.rotation %= 2.0 * std::f32::consts::PI;

        // Should prob make UI update last all the time.
        self.ui.update(ctx, self.player.hp);
        
        HandlerMessage::Keep
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        self.level.draw(ctx)?;

        self.player.draw(ctx)?;

        for blob in &self.blob {
            blob.draw(ctx)?;
        }

        let dp = graphics::DrawParam::default()
            .src(graphics::Rect::new(0.0, 0.0, 1.0, 1.0))
            .dest([736f32, 536f32])
            .offset([0.5, 0.5])
            .scale([2.0, 2.0])
            .rotation(self.rotation)
            .color(graphics::Color::new(
                1.0 - self.rotation / (2.0 * std::f32::consts::PI),
                self.rotation / (2.0 * std::f32::consts::PI),
                self.rotation / (2.0 * std::f32::consts::PI),
                1.0,
            ));
        graphics::draw(ctx, &self.sprite, dp)?;

        let dp = graphics::DrawParam::default()
            .dest([736f32, 64f32])
            .offset([0.5, 0.5])
            .scale([2.0, 2.0])
            .rotation(self.rotation);
        graphics::draw(ctx, &self.animated, dp)?;
        
        self.ui.draw(ctx);

        // This presents the contents of ctx to the game.
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) -> HandlerMessage {
        match key {
            KeyCode::P => {
                println!("Pause? Maybe latter.");
                HandlerMessage::Keep
            },
            _ => HandlerMessage::Keep
        }
    }
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
         // create player
        let mut player = Player::new(ctx);
        player.move_location(150f32, 150f32);
        let hp = player.hp;

        // create blobs (ie enemies)
        let mut blob = Vec::new();
        blob.push(Blob::new(ctx, 250.0, 250.0));
        blob.push(Blob::new(ctx, 250.0, 350.0));
        blob.push(Blob::new(ctx, 250.0, 150.0));

        // build level
        let img = graphics::Image::new(ctx, "/testwalls.png").unwrap();
        let mut lb = LevelBuilder::new(ctx, None);
        lb.set_tile_image(
            0usize,
            &Sprite::new(&img, graphics::Rect::new(0f32, 0f32, 64f32, 64f32)).unwrap(),
        )
        .unwrap();
        lb.set_tile_image(
            1usize,
            &Sprite::new(&img, graphics::Rect::new(64f32, 0f32, 64f32, 64f32)).unwrap(),
        )
        .unwrap();
        lb.set_tile_image(
            2usize,
            &Sprite::new(&img, graphics::Rect::new(128f32, 0f32, 64f32, 64f32)).unwrap(),
        )
        .unwrap();
        lb.set_tile_image(
            3usize,
            &Sprite::new(&img, graphics::Rect::new(192f32, 0f32, 64f32, 64f32)).unwrap(),
        )
        .unwrap();
        let level = lb.sample3();

        // demo sprites
        let img = graphics::Image::new(ctx, "/dapper-skeleton-sheet.png").unwrap();
        let sprite = Sprite::new(&img, graphics::Rect::new(0f32, 128f32, 64f32, 64f32)).unwrap();
        let animated = AnimatedBuilder::new(&img)
            .create_animated(graphics::Rect::new(0f32, 320f32, 64f32, 64f32), 6usize)
            .unwrap();

        // create state
        MainState {
            level,
            blob,
            player,
            ui: UI::new(ctx, "Adventurer".to_string(), hp),
            sprite,
            animated,
            rotation: 0f32,
        }
    }
}