//use ggez::graphics::*;

pub struct AnimatedBuilder {
    sheet: Image,
}

impl AnimatedBuilder {
    pub fn new(sheet: &Image) -> AnimatedBuilder {
        let sheet = sheet.clone();
        AnimatedBuilder{ sheet }
    }

    // creates animated sprites starting at (clip.x, clip.y) with width clip.w and height clip.h
    // may create less, but not more, frames then max_frames
    pub fn create_animated(&self, clip: Rect, max_frames: usize) -> GameResult<AnimatedSprite> {
        let mut clips = Vec::new();
        let mut xpos = clip.x;
        let mut max_frames = max_frames;
        while xpos + clip.w <= f32::from(self.sheet.width()) && max_frames > 0usize {
            clips.push(Rect::new(xpos, clip.y, clip.w, clip.h));
            xpos += clip.w;
            max_frames -= 1usize;
        }
        AnimatedSprite::new(&self.sheet, clips, None)
    }

    pub fn create_animated_once(&self, clip: Rect, max_frames: usize) -> GameResult<AnimatedSprite> {
        let ani = self.create_animated(clip, max_frames);
        match ani {
            Ok(mut mation) => {
                mation.once_animation();
                Ok(mation)
            }
            Err(_) => ani,
        }
    }
}