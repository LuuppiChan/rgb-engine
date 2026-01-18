use std::{sync::atomic::Ordering::Relaxed, time::Duration};

use nalgebra::Vector2;
use palette::{Srgb, num::ClampAssign};

use crate::{
    effect::Effect,
    keyboard::DeltaWatcher,
};

pub struct LocalPressBrightness {
    /// In case you want to clone this.
    pub delta_watcher: DeltaWatcher,
    /// The area of the affected keys.
    pub area: f64,
    /// Subtract from full brightness instead of adding to zero brightness.
    pub inverted: bool,
}

impl LocalPressBrightness {
    pub fn new(delta_watcher: DeltaWatcher, area: f64, inverted: bool) -> Self {
        Self {
            delta_watcher,
            area,
            inverted,
        }
    }
}

impl Effect for LocalPressBrightness {
    fn color(&self, _time: f64, pos_norm: Vector2<f64>) -> palette::Srgb<f64> {
        let pressed = self.delta_watcher.get_pressed_keys_mat_keys();
        let mut intensity = 0.0;
        for (key, key_pos) in pressed {
            if pos_norm.metric_distance(&key_pos.pos_norm) < self.area {
                intensity += key.distance.load(Relaxed) as f64 / 255.0;
            }
        }

        intensity.clamp_max_assign(1.0);

        if self.inverted {
            intensity = 1.0 - intensity;
        }

        Srgb::new(intensity, intensity, intensity)
    }
}
