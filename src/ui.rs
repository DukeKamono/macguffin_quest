use ggez::*;

pub struct UI {
	pub playerName: graphics::Text,
	
}

impl UI {
	pub fn new(ctx: &mut Context) -> UI {
		let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let hello = "hello".to_string();
        let text = graphics::Text::new((hello, font, 22.0));
		//let text = graphics::Text::new("hello");
		UI {
			playerName: text,
		}
	}
	
	pub fn update(&mut self, ctx: &mut Context) {
		
	}
	
	pub fn draw(&mut self, ctx: &mut Context) {
		let my_dest = nalgebra::Point2::new(13.0, 37.0);
		graphics::draw(ctx, &self.playerName, graphics::DrawParam::default().dest(my_dest) );
	}
}