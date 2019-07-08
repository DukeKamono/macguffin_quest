// Some Standard amethyst imports.
// Might not need all of them. Might need more.
use amethyst::{
    assets::Processor,
    ecs::{ReadExpect, Resources, SystemData},
    prelude::*,
    renderer::{
        pass::DrawFlat2DDesc, types::DefaultBackend, Factory, Format, GraphBuilder, GraphCreator,
        Kind, RenderGroupDesc, RenderingSystem, SpriteSheet, SubpassBuilder,
    },
    utils::application_root_dir,
    window::{ScreenDimensions, Window, WindowBundle},
};

// Refernces the custom_game_data file.
mod custom_game_data;

// bring in the crate custom_game_data and name it CustomGameDataBuilder
use crate::custom_game_data::CustomGameDataBuilder;

// Import the module 'systems'.
mod systems;

fn main() -> amethyst::Result<()> {
	//Default LoggerConfig https://docs-src.amethyst.rs/stable/amethyst/struct.Logger.html
	amethyst::start_logger(Default::default());
	
	// // Path to the Ron file. https://github.com/ron-rs/ron
	// let app_root = application_root_dir()?;
	// let display_config_path = app_root.join("resources").join("display_config.ron");
	
	// // Path to the inputs
	// let binding_path = app_root.join("resources").join("bindings_config.ron");
	
	// // Setup the input bundle.
	// let input_bundle = InputBundle::<StringBindings>::new()
    // .with_bindings_from_file(binding_path)?;
	
	// //Creating a WindowBundle using the Ron data.
	// let game_data = GameDataBuilder::default()
    // // The WindowBundle provides all the scaffolding for opening a window
    // .with_bundle(WindowBundle::from_config_path(display_config_path))?
	// // Add the transform bundle which handles tracking entity positions
	// .with_bundle(TransformBundle::new())?
	// // Add the UiBundle.
	// .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
	// // Add the inputs to the bundle.
	// .with_bundle(input_bundle)?
	// // Bundle the systems moudle. The input_system key itself is defined in the standard InputBundle.
	// .with(systems::PaddleSystem, "paddle_system", &["input_system"])
	 // // A Processor system is added to handle loading spritesheets.
    // .with(
        // Processor::<SpriteSheet>::new(),
        // "sprite_sheet_processor",
        // &[],
    // )
	// // Bundles the MoveBallsSystem.
	// .with(systems::MoveBallsSystem, "ball_system", &[])
	// // Bundles the BounceSystem.
    // .with(
        // systems::BounceSystem,
        // "collision_system",
        // &["paddle_system", "ball_system"],
    // )
	// // Bundles WinnerSystem.
	// .with(systems::WinnerSystem, "winner_system", &["ball_system"])
    // // The renderer must be executed on the same thread consecutively, so we initialize it as thread_local
    // // which will always execute on the main thread.
    // .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
        // ExampleGraph::default(),
    // ));

	// // Grabs the assets, game_data (Ron file), and the State
	// // then bundles them into a new Application.
	// // https://docs-src.amethyst.rs/stable/amethyst/type.Application.html
	// let assets_dir = app_root.join("assets/");
	// let mut game = Application::new(assets_dir, Pong::default(), game_data)?;

	// // The game will run until SimpleState return Trans::Quit or
	// // when all the states have been popped off the machine's stack.
	// game.run();
	
	let game_data = CustomGameDataBuilder::default()
    .with_running(ExampleSystem, "example_system", &[])
    .with_base_bundle(TransformBundle::new())?
    .with_base_bundle(UiBundle::<String, String>::new())?
    .with_base_bundle(RenderBundle::new(pipeline_builder, Some(display_config)))?
    .with_base_bundle(InputBundle::<String, String>::new())?;

	let mut game = Application::new(resources_directory, Main, game_data)?;
	game.run();
	
	
	Ok(())
}


// extern crate amethyst;

// // This graph structure is used for creating a proper `RenderGraph` for rendering.
// // A renderGraph can be thought of as the stages during a render pass. In our case,
// // we are only executing one subpass (DrawFlat2D, or the sprite pass). This graph
// // also needs to be rebuilt whenever the window is resized, so the boilerplate code
// // for that operation is also here.
// #[derive(Default)]
// struct ExampleGraph {
    // dimensions: Option<ScreenDimensions>,
    // dirty: bool,
// }

// impl GraphCreator<DefaultBackend> for ExampleGraph {
    // // This trait method reports to the renderer if the graph must be rebuilt, usually because
    // // the window has been resized. This implementation checks the screen size and returns true
    // // if it has changed.
    // fn rebuild(&mut self, res: &Resources) -> bool {
        // // Rebuild when dimensions change, but wait until at least two frames have the same.
        // let new_dimensions = res.try_fetch::<ScreenDimensions>();
        // use std::ops::Deref;
        // if self.dimensions.as_ref() != new_dimensions.as_ref().map(|d| d.deref()) {
            // self.dirty = true;
            // self.dimensions = new_dimensions.map(|d| d.clone());
            // return false;
        // }
        // return self.dirty;
    // }

    // // This is the core of a RenderGraph, which is building the actual graph with subpasses and target
    // // images. Using Rendy https://github.com/amethyst/rendy/blob/master/docs/graph.md
    // fn builder(
        // &mut self,
        // factory: &mut Factory<DefaultBackend>,
        // res: &Resources,
    // ) -> GraphBuilder<DefaultBackend, Resources> {
        // use amethyst::renderer::rendy::{
            // graph::present::PresentNode,
            // hal::command::{ClearDepthStencil, ClearValue},
        // };

        // self.dirty = false;

        // // Retrieve a reference to the target window, which is created by the WindowBundle
        // let window = <ReadExpect<'_, Window>>::fetch(res);
        // let dimensions = self.dimensions.as_ref().unwrap();
        // let window_kind = Kind::D2(dimensions.width() as u32, dimensions.height() as u32, 1, 1);

        // // Create a new drawing surface in our window
        // let surface = factory.create_surface(&window);
        // let surface_format = factory.get_surface_format(&surface);

        // // Begin building our RenderGraph
        // let mut graph_builder = GraphBuilder::new();
        // let color = graph_builder.create_image(
            // window_kind,
            // 1,
            // surface_format,
            // // clear screen to black
            // Some(ClearValue::Color([0.0, 0.0, 0.0, 1.0].into())),
        // );

        // let depth = graph_builder.create_image(
            // window_kind,
            // 1,
            // Format::D32Sfloat,
            // Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
        // );

        // // Create our first `Subpass`, which contains the DrawFlat2D and DrawUi render groups.
		// // We pass the subpass builder a description of our pass for construction
		// let pass = graph_builder.add_node(
			// SubpassBuilder::new()
				// .with_group(DrawFlat2DDesc::default().builder()) // Draws sprites
				// .with_group(DrawUiDesc::default().builder()) // Draws UI components
				// .with_color(color)
				// .with_depth_stencil(depth)
				// .into_pass(),
		// );

        // // Finally, add the pass to the graph
        // let _present = graph_builder
            // .add_node(PresentNode::builder(factory, surface, color).with_dependency(pass));

        // graph_builder
    // }
// }



// https://book.amethyst.rs/stable/controlling_system_execution/custom_game_data.html

// Some Standard amethyst imports.
// Might not need all of them. Might need more.
use amethyst::{
    assets::Processor,
    ecs::{ReadExpect, Resources, SystemData},
    prelude::*,
    renderer::{
        pass::DrawFlat2DDesc, types::DefaultBackend, Factory, Format, GraphBuilder, GraphCreator,
        Kind, RenderGroupDesc, RenderingSystem, SpriteSheet, SubpassBuilder,
    },
	core::ArcThreadPool,
    utils::application_root_dir,
    window::{ScreenDimensions, Window, WindowBundle},
};

struct Main;
struct Paused;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Paused {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        create_paused_ui(data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                delete_paused_ui(data.world);
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false); // false to say we should not dispatch running
        Trans::None
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Main {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        initialise(data.world);
    }

    fn handle_event(
        &mut self,
        _: StateData<CustomGameData>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                Trans::Push(Box::new(Paused))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true); // true to say we should dispatch running
        Trans::None
    }
}

pub struct CustomGameDataBuilder<'a, 'b> {
    pub core: DispatcherBuilder<'a, 'b>,
    pub running: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Default for CustomGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        CustomGameDataBuilder::new()
    }
}

impl<'a, 'b> CustomGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        CustomGameDataBuilder {
            core: DispatcherBuilder::new(),
            running: DispatcherBuilder::new(),
        }
    }

    pub fn with_base_bundle<B>(mut self, bundle: B) -> Result<Self, Error>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle.build(&mut self.core)?;
        Ok(self)
    }

    pub fn with_running<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.running.add(system, name, dependencies);
        self
    }
}

impl<'a, 'b> DataInit<CustomGameData<'a, 'b>> for CustomGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b> {
        // Get a handle to the `ThreadPool`.
        let pool = world.read_resource::<ArcThreadPool>().clone();

        let mut core_dispatcher = self.core.with_pool(pool.clone()).build();
        let mut running_dispatcher = self.running.with_pool(pool.clone()).build();
        core_dispatcher.setup(&mut world.res);
        running_dispatcher.setup(&mut world.res);

        CustomGameData { core_dispatcher, running_dispatcher }
    }
}

pub struct CustomGameData<'a, 'b> {
    core_dispatcher: Dispatcher<'a, 'b>,
    running_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> CustomGameData<'a, 'b> {
    /// Update game data
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            self.running_dispatcher.dispatch(&world.res);
        }
        self.core_dispatcher.dispatch(&world.res);
    }
}