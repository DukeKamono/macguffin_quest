use ggez::*;
use graphics::*;

struct Sprite {
    source_sheet: Image,
    draw_param: DrawParam,
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
        let dp = DrawParam::default().src(
            Rect::new(
                clip.x / sheet.width() as f32,
                clip.y / sheet.height() as f32,
                clip.w / sheet.width() as f32,
                clip.h / sheet.height() as f32,
            )
        );

        Ok(Sprite {
            source_sheet: sheet,
            draw_param: dp,
        })
    }
    fn draw(&self, ctx: &mut Context, xpos: f32, ypos: f32) -> GameResult {
        self.source_sheet.draw(
            ctx,
            self.draw_param.dest([xpos, ypos]),
        )?;
        Ok(())
    }
}

struct State {
    //img: Image,
    sprite: Vec<Sprite>,
}
impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let img = graphics::Image::new(ctx, "/dapper-skeleton-sheet.png")?;
        //let img = graphics::Image::new(ctx, "/dapper-skeleton-sheet-guides.png")?;

        let mut sprite = Vec::new();
        sprite.push(Sprite::new(&img, Rect::new(0f32, 128f32, 64f32, 64f32))?);
        sprite.push(Sprite::new(&img, Rect::new(128f32, 768f32, 64f32, 64f32))?);
        sprite.push(Sprite::new(&img, Rect::new(320f32, 256f32, 64f32, 64f32))?);

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
            s.draw(ctx, dest, dest)?;
            dest += 64f32;
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