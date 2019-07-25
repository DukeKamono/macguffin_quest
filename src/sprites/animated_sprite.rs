//use ggez::*;
//use ggez::graphics::*;
use std::time::Duration;

#[derive(Clone)]
pub enum AnimationState {
    _Paused(Duration), // accumulated duration when paused
    Loop,
}

#[derive(Clone)]
pub struct AnimatedSprite {
    sheet: Image,
    clips: Vec<Rect>,
    current_frame: usize,
    frame_rate: Duration, // how long per frame
    accumulated: Duration,
    state: AnimationState,
}

impl AnimatedSprite {
    pub fn new(sheet: &Image, clips: Vec<Rect>) -> GameResult<AnimatedSprite> {
        let sheet = sheet.clone();
        
        if clips.len() == 0usize {
            return Err(error::GameError::ResourceLoadError(
                format!(
                    "No clips to add to animated sprite"
                )
            ));
        }
        let mut rect = Vec::new();
        for c in clips {
            //if super::contains(&sheet.dimensions(), &c) {
            if contains(&sheet.dimensions(), &c) {
                rect.push(Rect::fraction(c.x, c.y, c.w, c.h, &sheet.dimensions()));
            } else {
                return Err(error::GameError::ResourceLoadError(
                    format!(
                        "Clip {:?} not contained in sheet",
                        c
                    )
                ));
            }
        }
        
        Ok(AnimatedSprite {
            sheet,
            clips: rect,
            current_frame: 0usize,
            frame_rate: Duration::new(0, 250_000_000),
            accumulated: Duration::new(0, 0),
            state: AnimationState::Loop,
        })
    }

    pub fn animate(&mut self, delta: Duration) {
        if let AnimationState::_Paused(_) = self.state {
            return;
        }
        self.accumulated += delta;
        if self.accumulated >= self.frame_rate {
            self.accumulated -= self.frame_rate;
            self.current_frame += 1usize;
        }
        if self.current_frame >= self.clips.len() {
            self.current_frame = 0usize;
        }
    }

    /*
    // may be useful one day ... just not today
    // good to have them though
    
    fn set_frame_rate(&mut self, rate: Duration) {
        self.frame_rate = rate;
    }

    fn set_frame(&mut self, frame_number: usize) {
        if frame_number < self.clips.len() {
            self.current_frame = frame_number;
        } else {
            self.current_frame = 0usize;
        }
    }

    fn loop_animation(&mut self) {
        if let AnimationState::Paused(a) = self.state {
            self.accumulated = a;
        }
        self.state = AnimationState::Loop;
    }

    fn pause_animation(&mut self) {
        if let AnimationState::Paused(_) = self.state {
            ()
        } else {
            self.state = AnimationState::Paused(self.accumulated);
        }
    }
    */
}

impl Drawable for AnimatedSprite {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let clip = self.clips[self.current_frame];
        let param = param.src(Rect::new(
            clip.x + clip.w * param.src.x,
            clip.y + clip.h * param.src.y,
            clip.w * param.src.w,
            clip.h * param.src.h,
        ));
        self.sheet.draw(ctx, param)
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        let mut dim = self.sheet.dimensions();
        dim.w *= self.clips[self.current_frame].w;
        dim.h *= self.clips[self.current_frame].h;
        Some(dim)
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.sheet.set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.sheet.blend_mode()
    }
}