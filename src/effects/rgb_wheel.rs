use std::f64::consts::PI;

use palette::{Hsv, IntoColor};

use crate::effect::Effect;

/// Spinny RGB
pub struct RgbWheel {
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
}

impl RgbWheel {
    pub fn new(speed: f64, scale: f64) -> Self {
        Self {
            speed,
            scale,
            hue_offset: 0.0,
            hue_range: 360.0,
        }
    }
}

impl Default for RgbWheel {
    fn default() -> Self {
        Self {
            hue_range: 360.0,
            hue_offset: 0.0,
            speed: 3.0,
            scale: 1.0,
        }
    }
}

impl Effect for RgbWheel {
    fn color(&self, time: f64, pos_norm: nalgebra::Vector2<f64>) -> palette::Srgb<f64> {
        let hue = ((pos_norm.x.atan2(pos_norm.y) * self.scale + time * self.speed) / (PI * 2.0))
            * self.hue_range
            + self.hue_offset;

        let hsv = Hsv::new(hue, 1.0, 1.0);

        hsv.into_color()
    }
}
