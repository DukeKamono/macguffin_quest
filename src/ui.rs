use ggez::*;

pub struct UI {
    pub player_name: graphics::Text,
    pub player_health: graphics::Text,
    pub dmg_text: Vec<Option<DmgText>>,
    //pub text_box: TextBox,
}

pub struct DmgText {
    point: nalgebra::Point2<f32>,
    text: graphics::Text,
    duration: f32,
}

//pub struct TextBox {
//	talker: graphics::Image,
//	text: graphics::Text,
//}

impl UI {
    pub fn new(ctx: &mut Context, name: String, health: f32) -> UI {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let p_name = graphics::Text::new((name, font, 22.0));
        let p_health = graphics::Text::new((health.to_string(), font, 22.0));
        
        UI {
            player_name: p_name,
            player_health: p_health,
            dmg_text: Vec::new(),
            //text_box: TextBox::new(ctx, "test".to_string()),
        }
    }
    
    pub fn update(&mut self, ctx: &mut Context, health: f32) {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        self.player_health = graphics::Text::new((health.to_string(), font, 22.0));
        
        // If there is a way to combine both of the next logic
        // pieces then go for it.
        self.dmg_text.retain(|x|
            if let Some(d) = x {
                if d.duration == 20.0 {
                    false
                }
                else {
                    true
                }
            }
            else
            {
                true
        });
            
        // This combined with the previous would be fun to think about.
        for dmg in &mut self.dmg_text {
            if let Some(d) = dmg {
                d.duration = d.duration + 1.0;
                d.point.y -= 1.0;
            }
        }
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
                d.draw(ctx);
            }
        }
    }
    
    //pub fn draw_text_box(&mut self, ctx: &mut Context) {
    //	
    //}
}

impl DmgText {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32, dmg: f32) -> DmgText {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let dmg_t = graphics::Text::new((dmg.to_string(), font, 22.0));
        
        DmgText {
            point: nalgebra::Point2::new(xpos + 5.0, ypos + 2.0),// The magic numbers help float over the object.
            text: dmg_t,
            duration: 0.0f32,
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        graphics::draw(ctx, &self.text, graphics::DrawParam::default().dest(self.point) ).expect("ERROR drawing Dmg Text");
    }
}

//impl TextBox {
//	pub fn new(ctx: &mut Context, text: String) -> TextBox {
//		let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
//        let t = graphics::Text::new((text, font, 22.0));
//		
//		TextBox {
//			text: t,
//			talker: graphics::Image::new(ctx, "/pong_spritesheet.png").unwrap(),
//		}
//	}
//	
//	pub fn update(&mut self, ctx: &mut Context) {
//		
//	}
//
//	pub fn draw(&self, ctx: &mut Context) {
//		let point = nalgebra::Point2::new(1000.0, 100.0);
//		graphics::draw(ctx, &self.text, graphics::DrawParam::default().dest(point) ).expect("ERROR drawing talk Text");
//	}
//}