use std::sync::atomic::Ordering::Relaxed;

use lerp::num_traits::Signed;
use palette::{Srgb, num::ClampAssign};

use crate::{effect::Effect, effects::analog::KeyFilter, keyboard::DeltaWatcher};

#[derive(Default)]
pub enum VelocityType {
    /// Both up and down motion counts to the velocity
    #[default]
    Both,
    /// Only up motion counts to the velocity
    Up,
    /// Only down motion counts to the velocity
    Down,
}

/// Lights up the keys around the key that has velocity.
/// This is not meant to be used as is since it will flash like crazy.
/// Instead this may be used to create energy injection for example.
pub struct Velocity {
    /// In case you want to clone this.
    pub delta_watcher: DeltaWatcher,
    /// Area around the key to light up
    pub area: f64,
    /// Intensity factor of the light that happens when a key has velocity.
    pub intensity: f64,
    /// What should trigger the velocity
    pub velocity_type: VelocityType,
    pub filter: KeyFilter,
}

impl Velocity {
    pub fn new(
        delta_watcher: DeltaWatcher,
        area: f64,
        intensity: f64,
        velocity_type: VelocityType,
    ) -> Self {
        Self {
            delta_watcher,
            area,
            intensity,
            velocity_type,
            filter: KeyFilter::All,
        }
    }
}

impl Effect for Velocity {
    fn color(&self, _time: f64, pos_norm: nalgebra::Vector2<f64>) -> Srgb<f64> {
        let pressed_keys = self.delta_watcher.get_pressed_keys_mat_keys();

        let mut intensity = 0.0;

        for (key, mat_key) in pressed_keys {
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
            if mat_key.pos_norm.metric_distance(&pos_norm) < self.area {
                let delta = key.delta_average.load(Relaxed);
                let delta = match self.velocity_type {
                    VelocityType::Both => delta.abs(),
                    VelocityType::Up if delta.is_negative() => delta.abs(),
                    VelocityType::Down if delta.is_positive() => delta,
                    _ => 0,
                };
                intensity += self.intensity * delta as f64;
            }
        }

        intensity.clamp_max_assign(1.0);

        Srgb::new(intensity, intensity, intensity)
    }
}
