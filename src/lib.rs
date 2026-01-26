mod bounds;
mod effect;
mod key;
mod process;
mod timer;

/// Some random built-in effects you can use to stack to your process.
pub mod effects;
/// All core keyboard communication components.
pub mod keyboard;

pub use effect::Effect;

pub use bounds::Bounds;

/// Everything related to core runtime and process logic.
pub mod runtime {
    pub use crate::key::*;
    pub use crate::process::*;
    pub use crate::timer::*;
}

/// Useful re-exports for creating effects
pub mod re_exports {
    pub use lerp;
    pub use nalgebra;
    pub use noise;
    pub use palette;
    pub use tween;
    pub use wooting_analog_wrapper;
    pub use wooting_rgb;
}
