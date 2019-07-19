use ggez::*;
use graphics::*;

// trait that is a wrapper for ggez::graphics::Drawable trait
// see Drawable documentation for trait's required methods
// https://docs.rs/ggez/0.5.0-rc.2/ggez/graphics/trait.Drawable.html
trait MyDrawable: Drawable {
    // do I need any other methods?
}

struct Sprite {
    source_sheet: Image,
    clip_rect: Rect,
}
impl Sprite {
    // provide reference to sprite sheet as source
    // provide clip rectangle as clip
    //  -- should be pixel values of source image you want drawn
    fn new(source: &Image, clip: Rect) -> GameResult<Sprite> {
        let sheet = source.clone();

        fn contains(bigger: &Rect, smaller: &Rect) -> bool {
            bigger.x <= smaller.x 
            && bigger.y <= smaller.y
            && bigger.w >= smaller.x + smaller.w
            && bigger.h >= smaller.y + smaller.h
        }
        if !contains(&sheet.dimensions(), &clip) {
            return Err(error::GameError::ResourceLoadError(format!("Clip {:?} not contained in source", clip)));
        }
        let rectangle = Rect::new(
            clip.x / sheet.width() as f32,
            clip.y / sheet.height() as f32,
            clip.w / sheet.width() as f32,
            clip.h / sheet.height() as f32,
        );

        Ok(Sprite {
            source_sheet: sheet,
            clip_rect: rectangle,
        })
    }
}
impl MyDrawable for Sprite {
    //none
}
impl Drawable for Sprite {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let dp = param.src(self.clip_rect);
        self.source_sheet.draw(ctx, dp)?;
        Ok(())
    }
    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        let clip = &self.clip_rect;
        let sheet = &self.source_sheet;
        Some(Rect::new(
            clip.x * sheet.width() as f32,
            clip.y * sheet.height() as f32,
            clip.w * sheet.width() as f32,
            clip.h * sheet.height() as f32,
        ))
    }
    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.source_sheet.set_blend_mode(mode);
    }
    fn blend_mode(&self) -> Option<BlendMode> {
        self.source_sheet.blend_mode()
    }
}

struct State {
    //img: Image,
    sprite: Vec<Sprite>,
}
impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        println!("Create State");

        let img = graphics::Image::new(ctx, "/dapper-skeleton-sheet.png")?;
        let mut sprite = Vec::new();
        sprite.push(Sprite::new(&img, Rect::new(0f32, 128f32, 64f32, 64f32))?);
        sprite.push(Sprite::new(&img, Rect::new(128f32, 768f32, 64f32, 64f32))?);
        sprite.push(Sprite::new(&img, Rect::new(320f32, 256f32, 64f32, 64f32))?);

        
        println!("Create State - finished");
        Ok(State {
            //img,
            sprite,
        })
    }
}
impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //update
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, BLACK);

        let mut dest = 0f32;
        for s in &self.sprite {
            draw(ctx, s, DrawParam::default().dest([dest, dest]))?;
            dest += 100f32;
        }

        present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() {
    // create context
    let (ctx, event_loop) = &mut ContextBuilder::new("collisions", "people")
        .window_setup(conf::WindowSetup::default().title("Collision Detection"))
        .add_resource_path(std::path::PathBuf::from("./resources/texture"))
        .build()
        .unwrap();
    // create state and game loop
    let state = &mut State::new(ctx).unwrap();
    // run loop
    match event::run(ctx, event_loop, state) {
        Ok(_) => println!("Clean loop exit"),
        Err(e) => println!("Error loop exit {}", e),
    };
    println!("Goodbye!");
}