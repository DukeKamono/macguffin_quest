use ggez::*;

trait Entity: event::EventHandler {
    // note: where self: Sized is needed for compiler to be able to make trait
    // object when trait function does not have a self parameter
    // https://doc.rust-lang.org/error-index.html#method-has-no-receiver
    //fn new() -> Self where Self: Sized;

    fn bounding_box(&self) -> graphics::Rect;

    fn display_bounding_box(&self) -> bool {
        false
    }

    fn collision(&self, e: &Entity) -> bool {
        let us = &self.bounding_box();
        let them = &e.bounding_box();
        us.overlaps(them)
    }
}

struct Circle {
    location: nalgebra::Point2<f32>,
    radius: f32,
}
impl Circle {
    fn new(x: f32, y:f32, r:f32) -> Circle {
        Circle{
            location: nalgebra::Point2::<f32>::new(x, y),
            radius: r,
        }
    }
}
impl Entity for Circle {    
    fn bounding_box(&self) -> graphics::Rect {
        graphics::Rect::new(
            self.location[0] - self.radius,
            self.location[1] - self.radius,
            self.radius * 2.0,
            self.radius * 2.0
        )
    }
}
impl event::EventHandler for Circle {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.location,
            self.radius,
            5.0,
            graphics::WHITE
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;

        let rectangle = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(1.0),
            self.bounding_box(),
            graphics::WHITE
        )?;
        graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;

        Ok(())
    }
}



struct Square {
    location: nalgebra::Point2<f32>,
    size: f32,
}
impl Square {
    fn new(x: f32, y:f32, s:f32) -> Square {
        Square{
            location: nalgebra::Point2::<f32>::new(x, y),
            size: s,
        }
    }
}
impl Entity for Square {    
    fn bounding_box(&self) -> graphics::Rect {
        graphics::Rect::new(
            self.location[0] - self.size,
            self.location[1] - self.size,
            self.size,
            self.size,
        )
    }
}
impl event::EventHandler for Square {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(),
            self.bounding_box(),
            graphics::WHITE
        )?;
        graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;

        Ok(())
    }
}

struct State {
    player: Box<Entity>,
    entities: Vec<Box<Entity>>,
}
impl State {
    fn new() -> State {
        let mut v = Vec::<Box<Entity>>::new();
        v.push(Box::new(Circle::new(20.0, 20.0, 20.0)));
        v.push(Box::new(Circle::new(400.0, 300.0, 50.0)));
        v.push(Box::new(Circle::new(100.0, 500.0, 100.0)));
        v.push(Box::new(Square::new(500.0, 50.0, 20.0)));
        v.push(Box::new(Square::new(700.0, 500.0, 100.0)));

        let p = Box::new(Square::new(400.0, 60.0, 50.0));

        State{ player: p, entities: v }
    }
}
impl event::EventHandler for State {
    // game loop to update logic... should do something...
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for i in &mut self.entities {
            // get a reference to contents of box<Entity>
            if i.collision(&*self.player) {
                println!("Collision detected");
            }
        }
        Ok(())
    }

    // draw things to screen
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear screen
        graphics::clear(ctx, graphics::BLACK);

        self.player.draw(ctx)?;

        for i in &mut self.entities {
            i.draw(ctx)?;
        }

        // display to screen
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
    let state = &mut State::new();
    // run loop
    match event::run(ctx, event_loop, state) {
        Ok(_) => println!("Clean loop exit"),
        Err(e) => println!("Error loop exit {}", e),
    };
    println!("Goodbye!");
}