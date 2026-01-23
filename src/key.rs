use nalgebra::Vector2;
use palette::Srgb;

#[derive(Default, Clone, Copy, Debug)]
pub struct Key {
    pub key: (u8, u8),
    /// A value from (0, 0) to (4, 14) representing the physical key position
    pub physical_position: Vector2<f64>,
    /// Normalized position based on physical_position
    pub pos_norm: Vector2<f64>,
    /// Normalized and aspect ratio corrected position of this key.
    /// x values range over 1
    pub pos_norm_aspect: Vector2<f64>,
    pub color: Srgb<f64>,
    pub color_blend_type: ColorBlendTypes,
}

impl Key {
    pub fn colors(&self) -> (u8, u8, u8) {
        let (r, g, b) = self.color.into_components();
        (
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        )
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Copy)]
pub enum ColorBlendTypes {
    /// Adds this color to the layer. `render_key.color += key.color`
    /// Good for a first background layer.
    #[default]
    Add,
    /// Same as add, but subtracts from the current render color.
    Sub,
    /// Multiplies render color.
    /// Each color channel is a multiplier to the render color of that channel.
    Mult,
    /// First multiplies the layer under this with the first value and then multiplies this layer
    /// with the second value.
    /// Then adds the two layers together.
    /// This gives good alpha layering.
    AlphaBlend(f64, f64),
    /// The render color is completely ignored and this key's color is used instead.
    Mask,
    /// This key is effectively ignored.
    Nothing,
}
