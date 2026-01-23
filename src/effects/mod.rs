#![allow(unused_imports)]
#![allow(dead_code)]

/// Analog based effects (requires analog to be enabled by runtime)
pub mod analog;
/// Perlin noise based effects
pub mod perlin;
mod placeholder;
mod random_colors;
mod rgb_wheel;
mod ripple;

pub use placeholder::PlaceholderEffect;
pub use random_colors::RandomColors;
pub use rgb_wheel::RgbWheel;
pub use ripple::Ripple;
