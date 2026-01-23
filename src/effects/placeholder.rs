use palette::Srgb;

use crate::Effect;

/// Placeholder effect that returns always black
#[derive(Default, Debug)]
pub struct PlaceholderEffect {}

impl Effect for PlaceholderEffect {
    fn color(&self, _time: f64, _pos_norm: nalgebra::Vector2<f64>) -> Srgb<f64> {
        Srgb::new(0.0, 0.0, 0.0)
    }
}
