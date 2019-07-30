use ggez::*;
use std::time::Duration;

// user interface (ie player name and health for now)
pub struct UI {
    pub player_name: graphics::Text,
    pub player_health: graphics::Text,
}

impl UI {
    pub fn new(ctx: &mut Context, name: String, health: f32) -> UI {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let p_name = graphics::Text::new((name, font, 22.0));
        let p_health = graphics::Text::new((health.to_string(), font, 22.0));

        UI {
            player_name: p_name,
            player_health: p_health,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, health: f32) {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        self.player_health = graphics::Text::new((health.to_string(), font, 22.0));
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let player_name_dest = nalgebra::Point2::new(100.0, 10.0);
        graphics::draw(
            ctx,
            &self.player_name,
            graphics::DrawParam::default().dest(player_name_dest),
        )
        .expect("ERROR drawing player name");

        let player_health_dest = nalgebra::Point2::new(100.0, 30.0);
        graphics::draw(
            ctx,
            &self.player_health,
            graphics::DrawParam::default().dest(player_health_dest),
        )
        .expect("ERROR drawing player health");
    }
}


/// Floating text (primarily for damage)

const FLOAT_SPEED: f64 = 5f64; // move 5 units every second

pub struct DmgText {
    point: nalgebra::Point2<f32>,
    text: graphics::Text,
    duration: Duration,
}

impl DmgText {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32, dmg: f32) -> DmgText {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let dmg_t = graphics::Text::new((dmg.to_string(), font, 22.0));

        DmgText {
            point: nalgebra::Point2::new(xpos + 5.0, ypos + 2.0), // The magic numbers help float over the object.
            text: dmg_t,
            duration: Duration::new(0, 0),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.duration += delta;
        let yinc = timer::duration_to_f64(delta) / FLOAT_SPEED;
        self.point.y -= yinc as f32;
    }

    pub fn live(&self) -> bool {
        self.duration < Duration::from_millis(100000)
    }

    pub fn draw(&self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.text,
            graphics::DrawParam::default().dest(self.point),
        )
        .expect("ERROR drawing Dmg Text");
    }
}
