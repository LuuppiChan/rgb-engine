use nalgebra::Vector2;
use palette::Srgb;

/// Main trait for all kinds of effects.
/// Mostly stateless since this doesn't allow mutation.
/// However with some extra functions this can have state, like the ripple effect.
pub trait Effect {
    /// Get color of the point based on space and time.
    fn color(&self, time: f64, pos_norm: Vector2<f64>) -> Srgb<f64>;
}
