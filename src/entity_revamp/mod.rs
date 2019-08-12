use std::time::Duration;

use ggez::{Context, GameResult};
use ggez::graphics::{BlendMode, Drawable, DrawParam, Image, Rect};

use crate::sprite_revamp::{Sprite, SpriteBuilder};

mod ai_revamp;

#[derive(Clone)]
pub struct Entity {
    sprite: Sprite,
    /*
        name
        type (enum for quick oversimplifications)
            - player
            - item
            - enemy
            - etc
        Stats
            - lvl
            - exp
            - health and health maximum
            - magic and magic maximum
            - attack damage
            - magic damage
            - defense
            - speed
        location (give drawparam and methods to alter)
        hitbox
        animations (with animated sprite to use)
        state (attacking, moving, damaged, dead, etc)
            - could make enum containing needed information
            - like attacking could have attack's hitbox and animation cooldown
        items (ie have the macguffin)
        floating text
        visible
        AI function (how act/update)
        sight radius
    */
}

impl Entity {
    pub fn new(ctx: &mut Context) -> Entity {
        let img = Image::new(ctx, "/dapper-skeleton-sheet.png").unwrap();
        // Duration::new(1u64, 500_000_000u32)
        let sprite = SpriteBuilder::new(&img)
            //.add_frames_going_up(4usize, Rect::new(0f32, 256f32, 64f32, 64f32), None, None, None)
            .add_frames_going_down(4usize, Rect::new(0f32, 576f32, 64f32, 64f32), None, None, None)
            //.add_frames_going_left(9usize, Rect::new(576f32, 0f32, 64f32, 64f32), None, None, None)
            //.add_frames_going_right(6usize, Rect::new(0f32, 768f32, 64f32, 64f32), None, None, None)
            //.add_frame(Rect::new(64f32, 768f32, 64f32, 64f32), None, None, None)
            //.build_sprite()
            .build_looping_sprite()
            .unwrap();
        Entity{
            sprite
        }
    }

    pub fn update(&mut self, delta: Duration) {
        // call ai
        // ai should deal damage... or at least spawn attack
        self.sprite.update(delta);
    }

    // translate

    // take damage

    // collision detection

    // is alive
}

/*
Should it be directly drawable
or should it really be drawn with own methods...
*/
impl Drawable for Entity {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult{
        self.sprite.draw(ctx, param)
    }
    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        self.sprite.dimensions(ctx)
    }
    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.sprite.set_blend_mode(mode)
    }
    fn blend_mode(&self) -> Option<BlendMode> {
        self.sprite.blend_mode()
    }
}


/*
Entity Builder
builds an entity like player or skeleton
*/
pub struct EntityBuilder {
    // fields
}