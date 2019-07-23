use ggez::*;
use ggez::graphics::*;

pub struct Sprite {
    sheet: Image,
    clip: Rect,
}

// helper function to test if smaller rectangle is inside a bigger rectangle
fn contains(bigger: &Rect, smaller: &Rect) -> bool {
    bigger.x <= smaller.x
        && bigger.y <= smaller.y
        && bigger.w >= smaller.x + smaller.w
        && bigger.h >= smaller.y + smaller.h
}

impl Sprite {
    pub fn new(sheet: &Image, clip: Rect) -> GameResult<Sprite> {
        let sheet = sheet.clone();

        if !contains(&sheet.dimensions(), &clip) {
            return Err(error::GameError::ResourceLoadError(
                format!(
                    "Clip {:?} not contained in sheet",
                    clip
                )
            ));
        }

        let clip = Rect::fraction(clip.x, clip.y, clip.w, clip.h, &sheet.dimensions());

        Ok(Sprite { sheet, clip })
    }
}

impl Drawable for Sprite {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let param = param.src(Rect::new(
            self.clip.x + self.clip.w * param.src.x,
            self.clip.y + self.clip.h * param.src.y,
            self.clip.w * param.src.w,
            self.clip.h * param.src.h,
        ));
        self.sheet.draw(ctx, param)
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        let mut dim = self.sheet.dimensions();
        //dim.x *= self.clip.x;
        //dim.y *= self.clip.y;
        dim.w *= self.clip.w;
        dim.h *= self.clip.h;
        Some(dim)
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.sheet.set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.sheet.blend_mode()
    }
}