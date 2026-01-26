use std::time::Duration;

use rgb_engine::{
    Effect,
    keyboard::get_matrix,
    re_exports::{
        nalgebra::Vector2,
        palette::{Hsv, IntoColor, Srgb},
    },
    runtime::{Process, Runtime},
};

fn main() {
    Runtime::new(false).run(&mut WaveProcess::new(Wave::new(-0.2, 0.4)));
}

struct Wave {
    speed: f64,
    scale: f64,
}

impl Wave {
    fn new(speed: f64, scale: f64) -> Self {
        Self { speed, scale }
    }
}

impl Effect for Wave {
    fn color(&self, time: f64, pos_norm: Vector2<f64>) -> Srgb<f64> {
        let hue = (pos_norm.x * self.scale + time * self.speed).sin() * 360.0;
        Hsv::new(hue, 1.0, 1.0).into_color()
    }
}

struct WaveProcess {
    effect: Wave,
}

impl WaveProcess {
    fn new(effect: Wave) -> Self {
        Self { effect }
    }
}

impl Process for WaveProcess {
    type Owner = Runtime<Self>;

    fn init(&mut self, runtime: &mut Self::Owner) {
        runtime.create_layer(0, get_matrix());
    }

    fn process(&mut self, runtime: &mut Self::Owner, _delta: Duration) {
        let elapsed = runtime.start.elapsed().as_secs_f64();
        for key in runtime.get_layer(0).as_flattened_mut() {
            key.color = self.effect.color(elapsed, key.pos_norm);
        }

        runtime.update_keyboard();
    }
}
