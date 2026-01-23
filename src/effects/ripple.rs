use std::time::Duration;

use nalgebra::Vector2;
use palette::Srgb;

use crate::{effect::Effect, keyboard::DeltaWatcher};

/// Ripple effect using the key press events from DeltaWatcher
pub struct Ripple {
    /// In case you want to clone this key_delta instance
    pub delta_watcher: DeltaWatcher,
    ripples: Vec<RippleEvent>,
    /// Max lifetime of a ripple in seconds
    pub max_lifetime: f64,
    /// Speed multiplier of the ripple
    pub speed: f64,
    /// How fast should the ripple decay
    pub decay: f64,
    /// How wide the ripple should be
    pub width: f64,
    /// true means that this darkens when multiplied with the underlying layer.
    /// false means that this brightens when multiplied with the underlying layer.
    pub inverted: bool,
}

impl Ripple {
    pub fn new(
        delta_watcher: DeltaWatcher,
        max_lifetime: f64,
        speed: f64,
        decay: f64,
        width: f64,
        inverted: bool,
    ) -> Self {
        Self {
            delta_watcher,
            ripples: Vec::new(),
            max_lifetime,
            speed,
            decay,
            width,
            inverted,
        }
    }

    pub fn update(&mut self, time: f64) {
        for (key, mat_key) in self.delta_watcher.get_pressed_keys_mat_keys() {
            if key.just_pressed() {
                self.ripples.push(RippleEvent {
                    origin: mat_key.pos_norm_aspect,
                    start_time: time,
                });
            }
        }

        self.ripples
            .retain(|ripple| time - ripple.start_time < self.max_lifetime);
    }

    fn wave(&self, x: f64) -> f64 {
        let v = 1.0 - (x.abs() / self.width);
        v.clamp(0.0, 1.0)
    }
}

impl Effect for Ripple {
    fn color(&self, time: f64, pos_norm: nalgebra::Vector2<f64>) -> palette::Srgb<f64> {
        let mut intensity = 0.0;

        for ripple in self.ripples.iter() {
            let elapsed = time - ripple.start_time;
            if elapsed < 0.0 {
                continue;
            }

            let radius = self.speed * elapsed;
            let dist = (pos_norm - ripple.origin).norm();
            let wave_strength = self.wave(dist - radius);
            let decay = (-self.decay * elapsed).exp();

            intensity += wave_strength * decay;
        }

        intensity = intensity.clamp(0.0, 1.0);
        if self.inverted {
            intensity = 1.0 - intensity;
        }

        Srgb::new(intensity, intensity, intensity)
    }
}

struct RippleEvent {
    origin: Vector2<f64>,
    start_time: f64,
}
