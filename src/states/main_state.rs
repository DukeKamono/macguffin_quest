use super::entities;
use entities::{CollideEntity, DrawableEntity};
use entities::player::playerstruct::Player;
use entities::enemies::{ai::AITypes,blob::Blob, skeleton::Skeleton, ghost::Ghost, boss::Boss, enemiesstruct::*};
use entities::environment::{level::Level, level_builder::LevelBuilder};
use entities::items::{macguffin::Macguffin, potions::Potions};
use entities::npcs::macguffin_man::MacguffinMan;

use super::ui::UI;

/// Game state for game (where actual gameplay happens!).
/// Can transition to the pause menu.
pub struct MainState {
    ui: UI,
    player: Player,
    enemies: Enemies,
    level: Level,
	macguffin: Option<Macguffin>,
	potions: Vec<Potions>,
	//npcs: Vec<NPCs>,
	macguffin_man: MacguffinMan
}

/// Implement CustomEventHandler from macguffin_quest::states::CustomEventHandler.
/// Allows the state machine to pass on information.
impl CustomEventHandler for MainState {
    /// Updates MainState...
    /// Which updates everything relevant to the game (like player, enemies, level, etc)
    fn update(&mut self, ctx: &mut Context) -> HandlerMessage {
        let delta = timer::delta(ctx);

        let playerx = self.player.x;
        let playery = self.player.y;
		
		// Collision with potions
		for p in &mut self.potions {
			if self.player.collision(p) && self.player.stats.hp < self.player.stats.max_hp {
				self.player.pick_up(ctx, (self.player.stats.max_hp - self.player.stats.hp).to_string());
				self.player.stats.hp = self.player.stats.max_hp;
				p.used = true;
			}
		}
		
		// Collision with macguffin
		if let Some(mac) = &self.macguffin {
			if mac.collision(&self.player) {
				self.player.macguffin = true; // Make a inventory system later
				self.enemies.push(Box::new(Boss::new(ctx, 1000.0, 1000.0, AITypes::Boss)));
				self.player.pick_up(ctx, "You Picked Up The MacGuffin!\nA Boss Has Appeared!".to_string());
			}
		}
		
		// Remove the macguffin.
		if self.player.macguffin {
			self.macguffin = None;
		}

		// another check because of text being setup before ui.update()
		if self.player.collision(&self.macguffin_man) {
			if self.player.macguffin {
				self.macguffin_man.talk(ctx, "You Found The MacGuffin!".to_string());
			}
			else {
				self.macguffin_man.talk(ctx, "Please Bring Me The MacGuffin!".to_string());
			}
		}
		self.macguffin_man.update(delta);
        self.enemies.update(ctx, delta, &mut self.player, &self.level);
		self.player.update(ctx, delta);
		
		// This could move into the player struct.
		// Needs to be after the player.update()
        if self.player.collision(&self.level) {
            self.player.move_location(playerx, playery);
        }
		
		// Update potions
		self.potions.retain(|t| !t.used);
		
        // Should prob make UI update last all the time.
        self.ui.update(ctx, self.player.stats.hp, self.player.stats.max_hp, self.player.stats.mp, self.player.stats.max_mp, self.player.stats.lv);
        
        // Should prob have it delayed untill after death animation...
        if self.player.stats.hp <= 0.0 {
            let state = Box::new(GameOverState::new(ctx));
            HandlerMessage::Change(state)
        }
		else if self.player.collision(&self.macguffin_man) {
			if self.player.macguffin {
				let state = Box::new(VictoryState::new(ctx));
				HandlerMessage::Change(state)
			}
			else {
				HandlerMessage::Keep
			}
		}
        else {
            HandlerMessage::Keep
        }
    }

    /// Draws MainState...
    /// Which draws everything relevant to the game (like player, enemies, level, etc)
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
		
		if let Some(mac) = &self.macguffin {
            mac.draw(ctx).expect("Failed to draw macguffin.");
        }
		
		self.macguffin_man.draw(ctx)?;
		
		for p in &self.potions {
			p.draw(ctx)?;
		}
        
        // reset screen coordinates for drawing UI
        MainState::set_screen_coordinates(ctx, 0f32, 0f32)?;
        
        self.ui.draw(ctx);

        // This presents the contents of ctx to the game.
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    /// Processes a key down event.
    /// This is where transitioning to pause state occurs.
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
    /// Creates a new MainState with player value
    pub fn new(ctx: &mut Context, chosen_player: String) -> MainState {
         // create player
        let mut player = Player::new(ctx, chosen_player);
        player.move_location(150f32, 150f32);
        let hp = player.stats.hp;
        let max_hp = player.stats.max_hp;
        let mp = player.stats.mp;
        let max_mp = player.stats.max_mp;
        let lv = player.stats.lv;

        // create enemies
        let mut e = Enemies::new();
        e.push(Box::new(Blob::new(ctx, 700.0, 250.0, AITypes::MeleeDirect)));
        e.push(Box::new(Blob::new(ctx, 700.0, 350.0, AITypes::MeleeDirect)));
        e.push(Box::new(Blob::new(ctx, 700.0, 150.0, AITypes::MeleeDirect)));
        e.push(Box::new(Skeleton::new(ctx, 950.0, 300.0, AITypes::MeleeLineOfSight)));
        e.push(Box::new(Skeleton::new(ctx, 950.0, -350.0, AITypes::MeleeLineOfSight)));
        e.push(Box::new(Skeleton::new(ctx, 1200.0, -350.0, AITypes::MeleeLineOfSight)));
        e.push(Box::new(Ghost::new(ctx, 600.0, 550.0, AITypes::MeleeLineOfSight)));
        e.push(Box::new(Ghost::new(ctx, 1600.0, 150.0, AITypes::MeleeLineOfSight)));
        e.push(Box::new(Ghost::new(ctx, 1600.0, 350.0, AITypes::MeleeLineOfSight)));
        e.push(Box::new(Ghost::new(ctx, 1600.0, 550.0, AITypes::MeleeLineOfSight)));

        // build level
        let img = graphics::Image::new(ctx, "/testwalls.png").unwrap();
        let mut lb = LevelBuilder::new(ctx, None);
        let level = lb.fromfile(ctx, &img, &"/BasicLevel.lvl".to_string());

		let mac = Macguffin::new(ctx, 1050.0, -650.0);
		
		let mut pot = Vec::new();
		pot.push(Potions::new(ctx, 1800.0, 350.0));
		pot.push(Potions::new(ctx, 250.0, 250.0));
		
		let npc = MacguffinMan::new(ctx, 250.0, 350.0);
		
        // create state
        MainState {
            level,
            enemies: e,
            player,
            ui: UI::new(ctx, "Adventurer".to_string(), hp, max_hp, mp, max_mp, lv),
			macguffin: Some(mac),
			potions: pot,
			macguffin_man: npc,
        }
    }

    /// Useful helper function for centering the screen on a given point.
    /// How the camera follows the player
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

// No good unit tests... very little (ie none) of this is non-trivial