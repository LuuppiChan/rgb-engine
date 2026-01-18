mod effect;
mod key;
mod process;
mod processes;
mod timer;

/// Some random built-in effects you can use to stack to your process.
pub mod effects;
/// All core keyboard communication components.
pub mod keyboard;

pub use effect::Effect;

/// Everything related to core runtime and process logic.
pub mod runtime {
    pub use crate::key::*;
    pub use crate::process::*;
    pub use crate::timer::*;
}

/// Includes a couple full examples I made.
pub mod examples {
    pub use crate::processes::*;
}

/// Useful re-exports for creating effects
pub mod re_exports {
    pub use lerp;
    pub use nalgebra;
    pub use noise;
    pub use palette;
    pub use tween;
    pub use wooting_analog_wrapper;
    pub use wooting_rgb;
}

#[cfg(test)]
mod tests {
    use std::{sync::atomic::Ordering::Relaxed, time::Duration};

    use crate::{
        examples::PinkRipple,
        process::{Process, Runtime},
    };

    #[test]
    fn runtime_no_analog() {
        let _: Runtime<PinkRipple> = Runtime::new(false);
    }

    #[test]
    fn runtime_analog() {
        let runtime: Runtime<PinkRipple> = Runtime::new(true);
        runtime.delta_watcher.unwrap().exit.load(Relaxed);
    }

    // #[test]
    // fn update() {
    //     let mut runtime: Runtime<PinkRipple> = Runtime::new(false);
    //     // If this segfaults it means the rgb stuff is fucked
    //     // no idea why though
    //     runtime.update_keyboard();
    // }

    #[test]
    fn run_effect_once() {
        let mut runtime: Runtime<PinkRipple> = Runtime::new(true);
        let mut ripple = PinkRipple::default();
        ripple.init(&mut runtime);
        ripple.process(&mut runtime, Duration::from_millis(1));
    }
}
