use lerp::Lerp;
use nalgebra::Vector2;
use noise::{NoiseFn, Perlin, core::value};
use palette::{IntoColor, Srgb};

use crate::{effect::Effect, effects::perlin::Direction};

/// Recreation of Wooting's noise effect.
pub struct TwoColorNoise {
    noise: Perlin,
    /// Scale multiplier of the noise
    pub scale: f64,
    /// Speed multiplier of the noise
    pub speed: f64,
    /// Primary noise color
    pub primary: Srgb<f64>,
    /// Secondary noise color
    pub secondary: Srgb<f64>,
    /// Direction of the noise.
    /// Meaning that the noise texture moves in that direction.
    /// This is done by adding time to a direction.
    pub direction: Direction,
}

impl TwoColorNoise {
    pub fn new(
        seed: u32,
        scale: f64,
        speed: f64,
        primary: Srgb<f64>,
        secondary: Srgb<f64>,
        direction: Direction,
    ) -> Self {
        Self {
            noise: Perlin::new(seed),
            scale,
            speed,
            primary,
            secondary,
            direction,
        }
    }
}

impl Default for TwoColorNoise {
    fn default() -> Self {
        Self {
            noise: Perlin::new(0),
            scale: 0.3,
            speed: -0.15,
            primary: Srgb::new(0.0, 255.0, 0.0),
            secondary: Srgb::new(0.0, 0.0, 255.0),
            direction: Direction::Depth,
        }
    }
}

impl Effect for TwoColorNoise {
    fn color(&self, time: f64, pos_norm: Vector2<f64>) -> Srgb<f64> {
        let pos = self.speed * time;
        let value = match self.direction {
            Direction::Horizontal => self.noise.get([pos_norm.x + pos, pos_norm.y]),
            Direction::Vertical => self.noise.get([pos_norm.x, pos_norm.y + pos]),
            Direction::Depth => self.noise.get([pos_norm.x, pos_norm.y, pos]),
        };
        self.primary.lerp(self.secondary, value).into_color()
    }
}
