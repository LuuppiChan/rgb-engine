use std::cell::{Cell, RefCell};

use palette::Srgb;
use rand::{Rng, rng, rngs::ThreadRng};

use crate::{effect::Effect, effects::perlin::PerlinWave};

use super::perlin::Direction;

/// Truly random colours for every key
/// All it does is just generate a random colour for each colour request
pub struct RandomColors {
    rng: RefCell<ThreadRng>,
}

impl RandomColors {
    pub fn new() -> Self {
        Self {
            rng: RefCell::new(rng()),
        }
    }
}

impl Default for RandomColors {
    fn default() -> Self {
        Self::new()
    }
}

impl Effect for RandomColors {
    fn color(&self, _time: f64, _pos_norm: nalgebra::Vector2<f64>) -> palette::Srgb<f64> {
        let mut rng = self.rng.borrow_mut();
        let red: f64 = rng.random();
        let green: f64 = rng.random();
        let blue: f64 = rng.random();

        Srgb::new(red, green, blue)
    }
}
