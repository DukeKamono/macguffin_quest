use std::time::Duration;

use ggez::{Context, GameError, GameResult};
use ggez::graphics::{BlendMode, Drawable, DrawParam, Image, Rect};

use crate::countdowntimer_revamp::CountDownTimer;

#[derive(Clone)]
pub struct Frame {
    clip: Rect,
    duration: Duration,
}

impl Frame {
    pub fn new(clip: Rect, duration: Duration) -> Frame {
        Frame {
            clip,
            duration,
        }
    }
}

// helper function to test if smaller rectangle is inside a bigger rectangle
fn contains(bigger: &Rect, smaller: &Rect) -> bool {
    bigger.x <= smaller.x
        && bigger.y <= smaller.y
        && bigger.w >= smaller.x + smaller.w
        && bigger.h >= smaller.y + smaller.h
}

#[derive(Clone)]
pub struct Sprite {
    sprite: Image,
    frames: Vec<Frame>,
    current_frame: usize,
    timer: CountDownTimer,
    looping: bool,
    paused: bool,
}

impl Sprite {
    pub fn new(sheet: &Image, frames: &Vec<Frame>) -> GameResult<Sprite> {
        // validate frames
        if frames.len() == 0usize {
            // Error: no frames to create Sprite out of
            return Err(GameError::ResourceLoadError("Error: no frames to create sprite with!".to_string()));
        }
        if !frames.iter().all(|f| contains(&sheet.dimensions(), &f.clip)) {
            // Error: at least one clip rectangle exists outside of sprite sheet
            return Err(GameError::ResourceLoadError(format!("Error: frame(s) exist outside of sheet's dimensions!")));
        }

        Ok(Sprite{
            sprite: sheet.clone(),
            frames: frames.clone(),
            current_frame: 0usize,
            timer: CountDownTimer::new(frames[0usize].duration),
            looping: false,
            paused: false,
        })
    }

    pub fn update(&mut self, delta: Duration) {
        if !self.paused {
            self.timer.update(delta);
            if self.timer.has_elapsed() {
                if self.current_frame < self.frames.len() - 1usize {
                    self.current_frame += 1usize;
                    self.timer.set(self.frames[self.current_frame].duration);
                } else if self.looping {
                    self.current_frame = 0usize;
                    self.timer.set(self.frames[self.current_frame].duration);
                } else {
                    // do nothing (ie stall out on last frame)
                }
            }
        }
    }

    pub fn _frame_number(&self) -> usize {
        self.current_frame
    }

    pub fn _set_frame_number(&mut self, frame_number: usize) -> Result<(), String> {
        if frame_number < self.frames.len() {
            self.current_frame = frame_number;
            Ok(())
        } else {
            Err(format!("Frame number '{}' is outside expected range. Needed to be a value between 0 and {}", frame_number, self.frames.len() - 1usize))
        }
    }

    pub fn _restart(&mut self) {
        // if this ever panics... something has gone wrong... like sprite having no frames
        self._set_frame_number(0usize).unwrap();
    }

    pub fn _is_looping(&self) -> bool {
        self.looping
    }

    pub fn set_looping(&mut self, looping: bool) {
        self.looping = looping;
    }

    pub fn _play(&mut self) {
        self.paused = false;
    }

    pub fn _pause(&mut self) {
        self.paused = true;
    }
}

impl Drawable for Sprite {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        // needed in case of drawing only part of current sprite frame
        // makes assumption that param.src has values between 0 and 1 (which it "should" according to documentation...)
        let clip = self.frames[self.current_frame].clip;
        let mut clip = Rect::fraction(clip.x, clip.y, clip.w, clip.h, &self.sprite.dimensions());
        clip.x += clip.w * param.src.x;
        clip.y += clip.h * param.src.y;
        clip.w *= param.src.w;
        clip.h *= param.src.h;
        // actually draw the sprite
        self.sprite.draw(ctx, param.src(clip))
    }
    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(self.frames[self.current_frame].clip)
    }
    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.sprite.set_blend_mode(mode);
    }
    fn blend_mode(&self) -> Option<BlendMode> {
        self.sprite.blend_mode()
    }
}

// sprite builder based on generating frames using iterators