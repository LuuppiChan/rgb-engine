use std::time::Duration;

use crate::{
    effect::Effect,
    effects::{
        Ripple,
        perlin::{Direction, PerlinWave},
    },
    key::ColorBlendTypes,
    keyboard::{DeltaWatcher, get_matrix},
    process::{Process, Runtime},
};

/// My favourite effect recreated with even better colour changes and ripple.
pub struct PinkRipple {
    pub background: PerlinWave,
    pub ripple: Ripple,
}

impl Default for PinkRipple {
    fn default() -> Self {
        let mut background = PerlinWave::new(0, 0.5, 0.8);
        background.hue_range = 30.0;
        background.hue_offset = 300.0;
        background.direction = Direction::Depth;
        Self {
            background,
            ripple: Ripple::new(DeltaWatcher::dummy(), 5.0, 2.0, 2.0, 0.3, true),
        }
    }
}

impl Process for PinkRipple {
    type Owner = Runtime<Self>;

    fn init(&mut self, runtime: &mut Self::Owner) {
        self.ripple.delta_watcher = runtime
            .delta_watcher
            .clone()
            .expect("Please enable delta_watcher");
        let mut layer = get_matrix();
        for key in layer.as_flattened_mut() {
            key.color_blend_type = ColorBlendTypes::Mult;
        }
        runtime.create_layer(1, layer);
        runtime.create_layer(0, get_matrix());

        runtime.create_timer(Duration::from_millis(50), false, move |runtime, process| {
            let elapsed = runtime.start.elapsed().as_secs_f64();
            for key in runtime.get_layer(0).as_flattened_mut() {
                key.color = process.background.color(elapsed, key.pos_norm);
            }

            for key in runtime.get_layer(1).as_flattened_mut() {
                key.color = process.ripple.color(elapsed, key.pos_norm);
            }

            runtime.update_keyboard();
            true
        });
    }

    fn process(&mut self, runtime: &mut Self::Owner, _delta: Duration) {
        self.ripple.update(runtime.start.elapsed().as_secs_f64());
    }
}
