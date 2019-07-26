use ggez::*;

pub struct UI {
	pub player_name: graphics::Text,
	pub player_health: graphics::Text,
	pub dmg_text: Vec<Option<DmgText>>,
}

pub struct DmgText {
	point: nalgebra::Point2<f32>,
	text: graphics::Text,
	duration: f32,
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
		
		self.dmg_text.retain(|x|
			if let Some(d) = x {
				println!("{}", d.duration);
				if d.duration == 50.0 {
					false
				}
				else {
					//d.duration += 1.0;
					true
				}
			}
			else
			{
				true
			});
			
			for dmg in &self.dmg_text {
				match dmg {
					Some(d) => d.duration = d.duration + 1.0,
					None => (),
				};
			}
			
			// Update syntax
			//struct Point3d {
			//	x: i32,
			//	y: i32,
			//	z: i32,
			//}
			//
			//let mut point = Point3d { x: 0, y: 0, z: 0 };
			//point = Point3d { y: 1, .. point };
			
			
		//for dmg in &self.dmg_text {
		//	if let Some(d) = dmg {
		//		if d.duration == 10.0 {
		//			let index = self.dmg_text.iter().position(|x| x == dmg).unwrap();
		//			&self.dmg_text.remove(index);
		//			//&self.dmg_text.remove(d);
		//		}
		//		else {
		//			d.duration += 1.0;
		//		}
		//	}
		//}
		//self.dmg_text = Vec::new();
	}
	
	pub fn update_dmg_text(&mut self, ctx: &mut Context, posx: f32, posy: f32, dmg: f32) {
		self.dmg_text.push(Some(DmgText::new(ctx, posx, posy, dmg)));
	}
	
	pub fn draw(&mut self, ctx: &mut Context) {
		let player_name_dest = nalgebra::Point2::new(100.0, 10.0);
		graphics::draw(ctx, &self.player_name, graphics::DrawParam::default().dest(player_name_dest) ).expect("ERROR drawing player name");
		
		let player_health_dest = nalgebra::Point2::new(100.0, 30.0);
		graphics::draw(ctx, &self.player_health, graphics::DrawParam::default().dest(player_health_dest) ).expect("ERROR drawing player health");
		
		for dmg in &self.dmg_text {
			if let Some(d) = dmg {
				d.draw(ctx);//.expect("Error drawing dmg text");
			}
		}
	}
}

impl DmgText {
	pub fn new(ctx: &mut Context, xpos: f32, ypos: f32, dmg: f32) -> DmgText {
		let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let dmg_t = graphics::Text::new((dmg.to_string(), font, 22.0));
		
		DmgText {
			point: nalgebra::Point2::new(xpos, ypos),
			text: dmg_t,
			duration: 0.0f32,
		}
	}

	pub fn draw(&self, ctx: &mut Context) {
		graphics::draw(ctx, &self.text, graphics::DrawParam::default().dest(self.point) ).expect("ERROR drawing Dmg Text");
	}
}