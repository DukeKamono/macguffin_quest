use ggez::*;
use graphics::*;
use std::time::Duration;

// helper function to test if smaller rectangle is inside a bigger rectangle
fn contains(bigger: &Rect, smaller: &Rect) -> bool {
    bigger.x <= smaller.x
        && bigger.y <= smaller.y
        && bigger.w >= smaller.x + smaller.w
        && bigger.h >= smaller.y + smaller.h
}

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

        if !contains(&sheet.dimensions(), &clip) {
            return Err(error::GameError::ResourceLoadError(format!(
                "Clip {:?} not contained in source",
                clip
            )));
        }
        let dp = DrawParam::default().src(Rect::fraction(
            clip.x,
            clip.y,
            clip.w,
            clip.h,
            &sheet.dimensions(),
        ));

        Ok(Sprite {
            source_sheet: sheet,
            draw_param: dp,
        })
    }
    fn draw(&self, ctx: &mut Context, xpos: f32, ypos: f32) -> GameResult {
        self.source_sheet
            .draw(ctx, self.draw_param.dest([xpos, ypos]))?;
        Ok(())
    }
}

struct AnimateSprite {
    source_sheet: Image,
    draw_param: Vec<DrawParam>,
    current_frame: usize,
    frame_duration: Duration,       // duration of a single frame
    accumulated_duration: Duration, // how long current frame has been displayed
}
impl AnimateSprite {
    // provide reference to sprite sheet as source
    // provide vector of clip rectangles as clip
    //  -- should be pixel values of source image you want drawn
    fn new(source: &Image, clip: Vec<Rect>) -> GameResult<AnimateSprite> {
        let sheet = source.clone();

        let mut params = Vec::new();
        for c in clip {
            if !contains(&sheet.dimensions(), &c) {
                return Err(error::GameError::ResourceLoadError(format!(
                    "Clip {:?} not contained in source",
                    c
                )));
            }
            params.push(
                DrawParam::default()
                    .src(Rect::fraction(c.x, c.y, c.w, c.h, &sheet.dimensions()))
                    .scale([4.0, 4.0]),
            );
        }

        Ok(AnimateSprite {
            source_sheet: sheet,
            draw_param: params,
            current_frame: 0usize,
            frame_duration: std::time::Duration::new(0, 250_000_000),
            accumulated_duration: std::time::Duration::new(0, 0),
        })
    }
    fn update(&mut self, delta: Duration) -> GameResult {
        self.accumulated_duration += delta;
        if self.accumulated_duration >= self.frame_duration {
            self.accumulated_duration -= self.frame_duration;
            self.current_frame += 1usize;
        }
        if self.current_frame >= self.draw_param.len() {
            self.current_frame = 0usize;
        }
        Ok(())
    }
    fn draw(&self, ctx: &mut Context, xpos: f32, ypos: f32) -> GameResult {
        self.source_sheet
            .draw(ctx, self.draw_param[self.current_frame].dest([xpos, ypos]))?;
        Ok(())
    }
}

struct State {
    //img: Image,
    sprite: Vec<Sprite>,
    animated: AnimateSprite,
}
impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let img = graphics::Image::new(ctx, "/dapper-skeleton-sheet.png")?;
        //let img = graphics::Image::new(ctx, "/dapper-skeleton-sheet-guides.png")?;

        let mut sprite = Vec::new();
        sprite.push(Sprite::new(&img, Rect::new(0f32, 128f32, 64f32, 64f32))?);
        sprite.push(Sprite::new(&img, Rect::new(128f32, 768f32, 64f32, 64f32))?);
        sprite.push(Sprite::new(&img, Rect::new(320f32, 256f32, 64f32, 64f32))?);

        let mut frames = Vec::new();
        frames.push(Rect::new(0f32, 320f32, 64f32, 64f32));
        frames.push(Rect::new(64f32, 320f32, 64f32, 64f32));
        frames.push(Rect::new(128f32, 320f32, 64f32, 64f32));
        frames.push(Rect::new(192f32, 320f32, 64f32, 64f32));
        frames.push(Rect::new(256f32, 320f32, 64f32, 64f32));
        frames.push(Rect::new(320f32, 320f32, 64f32, 64f32));
        let animated = AnimateSprite::new(&img, frames)?;

        Ok(State {
            //img,
            sprite,
            animated,
        })
    }
}
impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = timer::delta(ctx);
        self.animated.update(delta)?;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, BLACK);

        let mut dest = 0f32;
        for s in &self.sprite {
            s.draw(ctx, dest, dest)?;
            dest += 64f32;
        }

        self.animated.draw(ctx, dest, dest)?;

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
