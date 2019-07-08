// use amethyst::{
    // core::timing::Time,
    // core::transform::Transform,
    // ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
// };

// // Grabs the ball out of the pong.rs file.
// // The ball should be in it's own file ideally.
// use crate::pong::Ball;

// // MoveBallsSystem struct
// pub struct MoveBallsSystem;

// // Adding a system trait to MoveBallsSystem.
// impl<'s> System<'s> for MoveBallsSystem {
    // type SystemData = (
        // ReadStorage<'s, Ball>,
        // WriteStorage<'s, Transform>,
        // Read<'s, Time>,
    // );

    // fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
        // // Move every ball according to its speed, and the time passed.
		// // We do a loop just in case there is more than one ball.
        // for (ball, local) in (&balls, &mut locals).join() {
			// // We use time.delta_seconds to achieve delta time. All objects move at the same speed.
            // local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            // local.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
        // }
    // }
// }