use noise::{NoiseFn, Perlin};
use palette::{Hsv, IntoColor};

use crate::{effect::Effect, effects::perlin::Direction};

/// Brightness multiplier based on perlin noise.
/// As for clamping the color to some range.
/// You can do it yourself after getting the value.
pub struct Brightness {
    noise: Perlin,
    pub speed: f64,
    pub scale: f64,
    /// Direction of the noise
    pub direction: Direction,
}

impl Brightness {
    pub fn new(seed: u32, speed: f64, scale: f64) -> Self {
        Self {
            noise: Perlin::new(seed),
            speed,
            scale,
            direction: Direction::Depth,
        }
    }
}

impl Default for Brightness {
    fn default() -> Self {
        Self {
            noise: Default::default(),
            speed: 0.3,
            scale: 0.6,
            direction: Direction::Depth,
        }
    }
}

impl Effect for Brightness {
    fn color(&self, time: f64, pos_norm: nalgebra::Vector2<f64>) -> palette::Srgb<f64> {
        let scaled_norm = pos_norm * self.scale;
        let pos = time * self.speed;

        let value = match self.direction {
            Direction::Horizontal => self.noise.get([scaled_norm.x + pos, scaled_norm.y]),
            Direction::Vertical => self.noise.get([scaled_norm.x, scaled_norm.y + pos]),
            Direction::Depth => self.noise.get([scaled_norm.x, scaled_norm.y, pos]),
        };

        let hsv = Hsv::new(0.0, 0.0, value);

        hsv.into_color()
    }
}
