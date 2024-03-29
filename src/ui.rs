use ggez::*;
use std::time::Duration;

// user interface (ie player name and health for now)
pub struct UI {
    pub player_name: graphics::Text,
    pub player_health: graphics::Text,
    pub player_mp: graphics::Text,
    pub player_level: graphics::Text,
}

impl UI {
    pub fn new(
        ctx: &mut Context,
        name: String,
        health: f32,
        max_health: f32,
        mp: u32,
        max_mp: u32,
        level: u32,
    ) -> UI {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let p_name = graphics::Text::new((name, font, 22.0));
        let p_health = graphics::Text::new((
            "hp: ".to_string() + &health.to_string() + &"/".to_string() + &max_health.to_string(),
            font,
            22.0,
        ));
        let p_mp = graphics::Text::new((
            "mp: ".to_string() + &mp.to_string() + &"/".to_string() + &max_mp.to_string(),
            font,
            22.0,
        ));
        let lev = graphics::Text::new(("Level: ".to_string() + &level.to_string(), font, 22.0));

        UI {
            player_name: p_name,
            player_health: p_health,
            player_mp: p_mp,
            player_level: lev,
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        health: f32,
        max_health: f32,
        mp: u32,
        max_mp: u32,
        level: u32,
    ) {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        self.player_health = graphics::Text::new((
            "hp: ".to_string() + &health.to_string() + &"/".to_string() + &max_health.to_string(),
            font,
            22.0,
        ));
        self.player_mp = graphics::Text::new((
            "mp: ".to_string() + &mp.to_string() + &"/".to_string() + &max_mp.to_string(),
            font,
            22.0,
        ));
        self.player_level =
            graphics::Text::new(("Level: ".to_string() + &level.to_string(), font, 22.0));
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        // make sure text queue is cleared
        graphics::draw_queued_text(
            ctx,
            graphics::DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("unable to clear queue");

        // queue player name for drawing
        let player_name_dest = nalgebra::Point2::new(100.0, 10.0);
        graphics::queue_text(
            ctx,
            &self.player_name,
            player_name_dest,
            Some(graphics::WHITE),
        );

        // queue player health for drawing
        let player_health_dest = nalgebra::Point2::new(100.0, 30.0);
        graphics::queue_text(
            ctx,
            &self.player_health,
            player_health_dest,
            Some(graphics::WHITE),
        );

        // queue player mp for drawing
        let player_mp_dest = nalgebra::Point2::new(100.0, 50.0);
        graphics::queue_text(ctx, &self.player_mp, player_mp_dest, Some(graphics::WHITE));

        // queue player level for drawing
        let player_level_dest = nalgebra::Point2::new(100.0, 70.0);
        graphics::queue_text(
            ctx,
            &self.player_level,
            player_level_dest,
            Some(graphics::WHITE),
        );

        // draw ui
        graphics::draw_queued_text(
            ctx,
            graphics::DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("Error Drawing UI");
    }
}

/// Floating text (primarily for damage)

const FLOAT_SPEED: f64 = 25f64; // move 25 units every second
const LIFETIME: Duration = Duration::from_millis(1000); // text lives one sec

pub struct FloatingText {
    point: nalgebra::Point2<f32>,
    text: graphics::Text,
    duration: Duration,
    color: graphics::Color,
}

impl FloatingText {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32, text: String, color: &str) -> FloatingText {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let t = graphics::Text::new((text, font, 22.0));

        let c = match color {
            "White" => graphics::WHITE,
            "Black" => graphics::BLACK,
            "Red" => graphics::Color::from_rgb(255u8, 0u8, 0u8),
            "Green" => graphics::Color::from_rgb(0u8, 128u8, 0u8),
            "Blue" => graphics::Color::from_rgb(0u8, 0u8, 255u8),
            _ => graphics::WHITE,
        };

        FloatingText {
            point: nalgebra::Point2::new(xpos + 5.0, ypos + 2.0), // The magic numbers help float over the object.
            text: t,
            duration: Duration::new(0, 0),
            color: c,
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.duration += delta;
        let yinc = timer::duration_to_f64(delta) * FLOAT_SPEED;
        self.point.y -= yinc as f32;
    }

    pub fn live(&self) -> bool {
        self.duration < LIFETIME
    }

    pub fn draw(&self, ctx: &mut Context) {
        graphics::queue_text(ctx, &self.text, self.point, Some(self.color));
        graphics::draw_queued_text(
            ctx,
            graphics::DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("Error Drawing FloatingText");
    }
}
