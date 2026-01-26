use nalgebra::Vector2;

/// A rectangle which can be used as a collider or as a mesh for effects.
/// Used in flappy bird as such.
#[derive(Debug, Default, Clone, Copy)]
pub struct Bounds {
    pub position: Vector2<f64>,
    pub size: Vector2<f64>,
}

impl Bounds {
    pub fn contains(&self, point: Vector2<f64>) -> bool {
        self.position.x < point.x
            && self.position.x + self.size.x > point.x
            && self.position.y < point.y
            && self.position.y + self.size.y > point.y
    }

    pub fn intersects(&self, other: &Bounds) -> bool {
        self.position.x < other.position.x + other.size.x
            && self.position.x + self.size.x > other.position.x
            && self.position.y < other.position.y + other.size.y
            && self.position.y + self.size.y > other.position.y
    }

    pub fn center(&self) -> Vector2<f64> {
        self.position + self.size * 0.5
    }
}
