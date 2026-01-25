mod area;
mod local_press_brightness;
mod velocity;

pub use area::*;
pub use local_press_brightness::*;
pub use velocity::*;

/// Change what keys effect and what not.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyFilter {
    /// Allow all keys
    #[default]
    All,
    /// Include these keys
    Included(Vec<(u8, u8)>),
    /// Exclude these keys
    Excluded(Vec<(u8, u8)>),
}
