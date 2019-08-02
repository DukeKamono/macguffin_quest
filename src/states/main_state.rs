// contains all the information on entities
//mod entities;
// get collison trait from entities
use super::entities::{CollideEntity, DrawableEntity};
// get player struct to use
use super::entities::player::player::Player;
// get blob struct to use
use super::entities::enemies::blob::Blob;
use super::entities::enemies::skeleton::Skeleton;
use super::entities::enemies::enemies::*;
// get wall struct to use
use super::entities::environment::{level::Level, level_builder::LevelBuilder};

//mod ui;
use super::ui::UI;

//mod sprites;
//use sprites::sprite::Sprite;
//use sprites::animated_sprite::*;
use super::sprites::*;

use crate::entities::enemies::ai::AITypes;

pub struct MainState {
    ui: UI,
    player: Player,
    enemies: Enemies,//Vec<Box<dyn Enemy>>,
    level: Level,
}

impl CustomEventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> HandlerMessage {
        let delta = timer::delta(ctx);

        let playerx = self.player.x;
        let playery = self.player.y;

        self.player.update(ctx, delta);

        if self.player.collision(&self.level) {
            self.player.move_location(playerx, playery);
        }

        self.enemies.update(ctx, delta, &mut self.player, &self.level);

        //for blob in &mut self.blob {
        //    blob.update(delta);
        //
        //    if blob.collision(&self.player) {
        //        self.player.take_dmg(blob.atk);
        //    }
        //
        //    if let Some(atk) = &self.player.atk_box {
        //        if blob.collision(atk) {
        //            blob.take_dmg(ctx, self.player.atk);
        //        }
        //    }
        //}


        // Should prob make UI update last all the time.
        self.ui.update(ctx, self.player.hp);
        
        HandlerMessage::Keep
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        
        // change screen coords so it seems like following player
        let hb = self.player.get_hitbox();
        let swh = graphics::drawable_size(ctx);
        MainState::set_screen_coordinates(ctx,
            self.player.x - hb.w / 2f32 - swh.0 / 2f32,
            self.player.y - hb.h / 2f32 - swh.1 / 2f32
        )?;

        self.level.draw(ctx)?;

        self.player.draw(ctx)?;

        self.enemies.draw(ctx)?;
        
        // reset screen coordinates for drawing UI
        MainState::set_screen_coordinates(ctx, 0f32, 0f32)?;
        
        self.ui.draw(ctx);

        // This presents the contents of ctx to the game.
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) -> HandlerMessage {
        match key {
            KeyCode::P => {
                let state = Box::new(PauseState::new(ctx));
                HandlerMessage::Spawn(state)
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
        //let mut blob = vec![Box::new(Blob::new(ctx, 250.0, 250.0))];
        let mut e = Enemies::new();
        e.push(Box::new(Blob::new(ctx, 250.0, 250.0, AITypes::MeleeDirect)));
        e.push(Box::new(Blob::new(ctx, 250.0, 350.0, AITypes::MeleeDirect)));
        e.push(Box::new(Blob::new(ctx, 250.0, 150.0, AITypes::MeleeDirect)));
        e.push(Box::new(Skeleton::new(ctx, 550.0, 350.0)));

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

        // create state
        MainState {
            level,
            enemies: e,
            player,
            ui: UI::new(ctx, "Adventurer".to_string(), hp),
        }
    }

    fn set_screen_coordinates(ctx: &mut Context, x: f32, y: f32) -> GameResult {
        let swh = graphics::drawable_size(ctx);
        let screen_shift = graphics::Rect::new(
            x,
            y,
            swh.0,
            swh.1);
        graphics::set_screen_coordinates(ctx, screen_shift)
    }
}