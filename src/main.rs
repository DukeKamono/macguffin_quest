use ggez::*;

mod states;
use states::StateMachine;

fn main() {
    // create a context to access hardware (also creates event loop)
    let c = ggez::conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) =
        ggez::ContextBuilder::new("macguffin_quest", "James M. & William O.")
            .add_resource_path(std::path::PathBuf::from("./resources/texture"))
            .add_resource_path(std::path::PathBuf::from("./resources/font"))
            .conf(c)
            .build()
            .unwrap();

    // create player
    let mut player = Player::new(ctx);
    player.move_location(150f32, 150f32);
	let hp = player.hp;

    // create blobs (ie enemies)
    let mut blob = Vec::new();
    blob.push(Blob::new(ctx, 250.0, 250.0));
    blob.push(Blob::new(ctx, 250.0, 350.0));
    blob.push(Blob::new(ctx, 250.0, 150.0));

    // build level
    let img = graphics::Image::new(ctx, "/testwalls.png").unwrap();
    let mut lb = LevelBuilder::new(ctx, None);
    lb.set_tile_image(
        0usize,
        &Sprite::new(&img, graphics::Rect::new(0f32, 0f32, 64f32, 64f32)).unwrap(),
    )
    .unwrap();
    lb.set_tile_image(
        1usize,
        &Sprite::new(&img, graphics::Rect::new(64f32, 0f32, 64f32, 64f32)).unwrap(),
    )
    .unwrap();
    lb.set_tile_image(
        2usize,
        &Sprite::new(&img, graphics::Rect::new(128f32, 0f32, 64f32, 64f32)).unwrap(),
    )
    .unwrap();
    lb.set_tile_image(
        3usize,
        &Sprite::new(&img, graphics::Rect::new(192f32, 0f32, 64f32, 64f32)).unwrap(),
    )
    .unwrap();
    let level = lb.sample3();

    // demo sprites
    let img = graphics::Image::new(ctx, "/dapper-skeleton-sheet.png").unwrap();
    let sprite = Sprite::new(&img, graphics::Rect::new(0f32, 128f32, 64f32, 64f32)).unwrap();
    let animated = AnimatedBuilder::new(&img)
        .create_animated(graphics::Rect::new(0f32, 320f32, 64f32, 64f32), 6usize)
        .unwrap();

    // create state
    let state = &mut MainState {
        level,
        blob,
        player,
		ui: UI::new(ctx, "Adventurer".to_string(), hp),
        sprite,
        animated,
        rotation: 0f32,
    };
	
	let state = &mut MainMenuState {
		ui: UI::new(ctx, "Adventurer".to_string(), 10.0),
	};

    // start game loop
    match ggez::event::run(ctx, event_loop, state) {
        Ok(_) => println!("Exiting Game."),
        Err(e) => println!("Run event loop broke! {}", e),
    }
}
