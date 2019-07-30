use ggez::*;
use ggez::graphics::*;

#[derive(Clone)]
pub struct Sprite {
    sheet: Image,
    clip: Rect,
}

impl Sprite {
    pub fn new(sheet: &Image, clip: Rect) -> GameResult<Sprite> {
        let sheet = sheet.clone();

        //if !super::contains(&sheet.dimensions(), &clip) {
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

    pub fn width(&self) -> f32 {
        self.clip.w * f32::from(self.sheet.width())
    }

    pub fn height(&self) -> f32 {
        self.clip.h * f32::from(self.sheet.height())
    }

    pub fn dimensions(&self) -> Option<Rect> {
        let mut dim = self.sheet.dimensions();
        dim.w *= self.clip.w;
        dim.h *= self.clip.h;
        Some(dim)
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
        self.dimensions()
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.sheet.set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.sheet.blend_mode()
    }
}