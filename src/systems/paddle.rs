// use amethyst::core::{math::RealField, Float, Transform};
// use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
// use amethyst::input::{InputHandler, StringBindings};

// // You'll have to mark PADDLE_HEIGHT as public in pong.rs
// use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

// // Unit struct PaddleSystem
// pub struct PaddleSystem;

// // System traid for PaddleSystem. Systems are like MonoBehaviour in Unity.
// impl<'s> System<'s> for PaddleSystem {
	// // Tuple SystemData
    // type SystemData = (
        // WriteStorage<'s, Transform>,
        // ReadStorage<'s, Paddle>,
        // Read<'s, InputHandler<StringBindings>>,
    // );

    // fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
		// // This will iterate over all entities that have both a Paddle and Transform attached to them,
		// // and give us access to the actual components,
		// // immutable for the Paddle and mutable for the Transform.
		// // Could use par_join instead of join for multiple threads.
        // for (paddle, transform) in (&paddles, &mut transforms).join() {
            // let movement = match paddle.side {
                // Side::Left => input.axis_value("left_paddle"),
                // Side::Right => input.axis_value("right_paddle"),
            // };
            // if let Some(mv_amount) = movement {
                // if mv_amount != 0.0 {
					// let scaled_amount = 1.2 * mv_amount as f32;
					// // Prevent off the edge paddles.
					// let paddle_y = transform.translation().y;
					// transform.set_translation_y(
						// (paddle_y + Float::from(scaled_amount))
							// .min(Float::from(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5))
							// .max(Float::from(PADDLE_HEIGHT * 0.5)),
					// );
					
					// // Don't want to go off the edge. Don't need this.
					// //transform.prepend_translation_y(scaled_amount);
					
					// // This commented out area was for testing if the button was pressed.
                    // // let side_name = match paddle.side {
                        // // Side::Left => "left",
                        // // Side::Right => "right",
                    // // };
                    // // println!("Side {:?} moving {}", side_name, mv_amount);
                // }
            // }
        // }
    // }
// }