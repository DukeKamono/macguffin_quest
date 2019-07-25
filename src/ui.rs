use ggez::*;

pub struct UI {
	pub player_name: graphics::Text,
	pub player_health: graphics::Text,
	pub dmg_text: Vec<Option<graphics::Text>>,
}

impl UI {
	pub fn new(ctx: &mut Context, name: String, health: f32) -> UI {
		let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let p_name = graphics::Text::new((name, font, 22.0));
		let p_health = graphics::Text::new((health.to_string(), font, 22.0));
		
		UI {
			player_name: p_name,
			player_health: p_health,
			dmg_text: Vec::new(),
		}
	}
	
	pub fn update(&mut self, ctx: &mut Context, health: f32) {
		let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
		self.player_health = graphics::Text::new((health.to_string(), font, 22.0));
	}
	
	pub fn draw(&mut self, ctx: &mut Context) {
		let player_name_dest = nalgebra::Point2::new(100.0, 10.0);
		graphics::draw(ctx, &self.player_name, graphics::DrawParam::default().dest(player_name_dest) ).expect("ERROR drawing player name");
		
		let player_health_dest = nalgebra::Point2::new(100.0, 30.0);
		graphics::draw(ctx, &self.player_health, graphics::DrawParam::default().dest(player_health_dest) ).expect("ERROR drawing player health");
	}
}