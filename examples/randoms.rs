use std::{thread::sleep, time::Duration};

use rand::{Rng, rngs::ThreadRng};
use rgb_engine::{
    keyboard::get_matrix,
    re_exports::{lerp::Lerp, palette::Srgb},
    runtime::{Process, Runtime},
};

fn main() {
    Runtime::new(true).run(&mut Randoms::default());
}

#[derive(Default)]
pub struct Randoms {
    rng: ThreadRng,
}

impl Randoms {
    pub fn get_random_color(&mut self) -> Srgb<f64> {
        Srgb::new(self.rng.random(), self.rng.random(), self.rng.random()) * 1.5
    }
}

impl Process for Randoms {
    type Owner = Runtime<Self>;

    fn init(&mut self, runtime: &mut Self::Owner) {
        assert!(
            runtime.delta_watcher.is_some(),
            "Please enable analog feature"
        );

        runtime.create_layer(0, get_matrix());
        runtime.create_timer(Duration::from_millis(20), false, |r, p| {
            let layer = r.get_layer(0).as_flattened_mut();
            let i = p.rng.random_range(0..layer.len());
            let key = &mut layer[i];
            key.color = p.get_random_color();
            true
        });

        runtime.create_timer(Duration::from_millis(50), false, |r, _p| {
            r.update_keyboard();
            true
        });
    }

    fn process(&mut self, runtime: &mut Self::Owner, delta: std::time::Duration) {
        sleep(Duration::from_millis(10).saturating_sub(delta));

        for key in runtime.get_layer(0).as_flattened_mut() {
            key.color = key
                .color
                .lerp(Srgb::new(0.0, 0.0, 0.0), delta.as_secs_f64() * 0.7);
        }

        let pressed = if let Some(delta_watcher) = &runtime.delta_watcher {
            let mut pressed = Vec::new();
            for key in delta_watcher.get_pressed_keys() {
                pressed.push(key.clone());
            }
            pressed
        } else {
            unreachable!()
        };

        for key in runtime.get_layer(0).as_flattened_mut() {
            if pressed.iter().any(|p| p.key == key.key && p.just_pressed()) {
                key.color = self.get_random_color();
            }
        }
    }
}
