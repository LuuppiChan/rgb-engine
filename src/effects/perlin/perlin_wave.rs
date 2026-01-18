use noise::{NoiseFn, Perlin};
use palette::{Hsv, IntoColor};

use crate::{effect::Effect, effects::perlin::Direction};

/// 2D/3D wave effect which uses perlin noise to get a color value.
pub struct PerlinWave {
    noise: Perlin,
    /// Speed of the wave.
    pub speed: f64,
    /// Scale multiplier for the wave.
    /// How fast the colors should change.
    pub scale: f64,
    /// Where to start at the hue spectrum. (0 to 360)
    pub hue_offset: f64,
    /// How wide should the RGB range be (from 0 to 360).
    /// 360 being the default meaning that it includes every color in the spectrum.
    pub hue_range: f64,
    /// Direction of the noise.
    /// Meaning that the noise texture moves in that direction.
    /// This is done by adding time to a direction.
    pub direction: Direction,
}

impl PerlinWave {
    pub fn new(seed: u32, speed: f64, scale: f64) -> Self {
        Self {
            noise: Perlin::new(seed),
            speed,
            scale,
            hue_offset: 0.0,
            hue_range: 360.0,
            direction: Direction::Horizontal,
        }
    }
}

impl Default for PerlinWave {
    fn default() -> Self {
        Self {
            noise: Perlin::default(),
            speed: -0.15,
            scale: 0.3,
            hue_offset: 0.0,
            hue_range: 360.0,
            direction: Direction::Depth,
        }
    }
}

impl Effect for PerlinWave {
    fn color(&self, time: f64, pos_norm: nalgebra::Vector2<f64>) -> palette::Srgb<f64> {
        let pos = time * self.speed;
        let scaled_norm = pos_norm * self.scale;
        let hue = match self.direction {
            Direction::Horizontal => self.noise.get([scaled_norm.x + pos, scaled_norm.y]),
            Direction::Vertical => self.noise.get([scaled_norm.x, scaled_norm.y + pos]),
            Direction::Depth => self.noise.get([scaled_norm.x, scaled_norm.y, pos]),
        };
        let hsv = Hsv::new(hue * self.hue_range + self.hue_offset, 1.0, 1.0);

        hsv.into_color()
    }
}
