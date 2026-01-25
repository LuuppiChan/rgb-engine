use std::sync::atomic::Ordering::Relaxed;

use palette::{Srgb, num::ClampAssign};

use crate::{effect::Effect, effects::analog::KeyFilter, keyboard::DeltaWatcher};

/// Lights up keys around the pressed key based on how down it's pressed.
pub struct Area {
    /// In case you want to clone this to another thing.
    pub delta_watcher: DeltaWatcher,
    /// Max area to light up
    pub area: f64,
    /// How much should the key light up when it's affected by one area.
    pub brightness: f64,
    pub filter: KeyFilter,
}

impl Area {
    pub fn new(delta_watcher: DeltaWatcher, area: f64, brightness: f64) -> Self {
        Self {
            delta_watcher,
            area,
            brightness,
            filter: KeyFilter::All,
        }
    }
}

impl Effect for Area {
    fn color(&self, _time: f64, pos_norm: nalgebra::Vector2<f64>) -> palette::Srgb<f64> {
        let pressed = self.delta_watcher.get_pressed_keys_mat_keys();

        let mut intensity = 0.0;
        for (key, mat_key) in pressed {
            match &self.filter {
                KeyFilter::All => (),
                KeyFilter::Included(items) => {
                    if !items.contains(&key.key) {
                        continue;
                    }
                }
                KeyFilter::Excluded(items) => {
                    if items.contains(&key.key) {
                        continue;
                    }
                }
            }
            if mat_key.pos_norm_aspect.metric_distance(&pos_norm)
                < self.area * (key.distance.load(Relaxed) as f64 / 255.0)
            {
                intensity += self.brightness;
            }
        }

        intensity.clamp_assign(0.0, 1.0);

        Srgb::new(intensity, intensity, intensity)
    }
}
