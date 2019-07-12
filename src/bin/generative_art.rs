// based on guide found at
// https://github.com/ggez/ggez/blob/master/docs/guides/GenerativeArt.md

use ggez::*;

enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}

struct State {
    shapes: Vec<Shape>,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for shape in &self.shapes {
            // Make the shape
            let mesh = match *shape {
                Shape::Rectangle(rect) => graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    graphics::WHITE,
                )?,
                Shape::Circle(origin, radius) => graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    origin,
                    radius,
                    0.1,
                    graphics::WHITE,
                )?,
            };
            // draw shape
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let mut shapes = Vec::new();
    // could do random stuff here (add in loop)
    // I just did not bother
    shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(
        10.0, 10.0, 50.0, 100.0,
    )));
    shapes.push(Shape::Circle(mint::Point2 { x: 400.0, y: 40.0 }, 30.0));
    let state = &mut State { shapes };
    let cb = ggez::ContextBuilder::new("generative_art", "awesome_person");
    let (ref mut ctx, ref mut event_loop) = &mut cb.build().unwrap();
    event::run(ctx, event_loop, state).unwrap();
    println!("Goodbye!");
}
