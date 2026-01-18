mod bounds;
mod game;

pub use bounds::Bounds;
pub use game::FlappyBird;

use crate::process::Runtime;

/// Run this full effect.
/// Returns if esc key is pressed.
/// This function is literally 2 lines btw.
pub fn run_flappy_bird() {
    let mut runtime = Runtime::new(true);
    runtime.run(&mut FlappyBird::default());
}
