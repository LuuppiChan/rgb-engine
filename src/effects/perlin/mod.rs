mod brightness;
mod perlin_wave;
mod two_color_noise;

pub use brightness::*;
pub use perlin_wave::*;
pub use two_color_noise::*;

/// Direction of the noise
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
    /// 3D!
    Depth,
}
