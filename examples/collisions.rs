use ggez::*;

trait MyDrawTrait {
    // https://doc.rust-lang.org/error-index.html#method-has-no-receiver
    fn new(ctx: &mut Context, xpos: f32, ypos: f32) -> Self
    where
        Self: Sized;
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn move_location(&mut self, xinc: f32, yinc: f32);
}
trait MyCollideTrait: MyDrawTrait {
    fn hit_box(&self) -> graphics::Rect;
    // not sure if this is right
    fn collision<T>(&self, other: &T) -> bool
    where
        T: MyCollideTrait;
}

struct Object {
    shape: graphics::Mesh,
    hit_box: graphics::Rect,
    x: f32,
    y: f32,
}
impl MyDrawTrait for Object {
    fn new(ctx: &mut Context, xpos: f32, ypos: f32) -> Object {
        // radius of circle
        let r = 50f32;
        // create hit box
        let hb = graphics::Rect::new(0.0, 0.0, r * 2.0, r * 2.0);
        // create mesh
        let circle = graphics::MeshBuilder::new()
            .circle(
                graphics::DrawMode::fill(),
                nalgebra::Point2::new(r, r),
                r,
                1.0,
                graphics::WHITE,
            )
            .rectangle(graphics::DrawMode::stroke(1.0), hb.clone(), graphics::WHITE)
            .build(ctx)
            .unwrap();
        // return new object
        Object {
            shape: circle,
            hit_box: hb,
            x: xpos,
            y: ypos,
        }
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(nalgebra::Point2::new(self.x, self.y));
        graphics::draw(ctx, &self.shape, dp)
    }
    fn move_location(&mut self, xinc: f32, yinc: f32) {
        self.x += xinc;
        self.y += yinc;
    }
}
impl MyCollideTrait for Object {
    fn hit_box(&self) -> graphics::Rect {
        let mut r = self.hit_box.clone();
        r.x = self.x;
        r.y = self.y;
        r
    }
    fn collision<T>(&self, other: &T) -> bool
    where
        T: MyCollideTrait,
    {
        self.hit_box().overlaps(&other.hit_box())
    }
}

struct State {
    player: Object,
    // should really be Vec<Box<MyCollideTrait>>
    // but that makes this example harder
    walls: Vec<Object>,
}
impl State {
    fn new(ctx: &mut Context) -> State {
        let p = Object::new(ctx, 0.0, 0.0);

        let mut v = Vec::new();
        v.push(Object::new(ctx, 350.0, 150.0));
        v.push(Object::new(ctx, 350.0, 250.0));
        v.push(Object::new(ctx, 350.0, 350.0));

        State {
            player: p,
            walls: v,
        }
    }
}
impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut xmov = 0f32;
        let mut ymov = 0f32;

        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Right) {
            xmov += 5.0;
        }
        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Left) {
            xmov += -5.0;
        }
        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Up) {
            ymov += -5.0;
        }
        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Down) {
            ymov += 5.0;
        }

        self.player.move_location(xmov, ymov);
        for wall in &self.walls {
            if self.player.collision(wall) {
                self.player.move_location(-xmov, -ymov);
            }
        }

        //println!("{:?}", self.player.hit_box());

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for wall in &self.walls {
            wall.draw(ctx)?;
        }
        self.player.draw(ctx)?;

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() {
    // create context
    let (ctx, event_loop) = &mut ContextBuilder::new("collisions", "people")
        .window_setup(conf::WindowSetup::default().title("Collision Detection"))
        .build()
        .unwrap();
    // create state and game loop
    let state = &mut State::new(ctx);
    // run loop
    match event::run(ctx, event_loop, state) {
        Ok(_) => println!("Clean loop exit"),
        Err(e) => println!("Error loop exit {}", e),
    };
    println!("Goodbye!");
}
