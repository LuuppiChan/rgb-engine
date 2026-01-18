# rgb-engine
An RGB effect engine for Wooting 60HE.
# Examples
Examples can be found inside the library and here's also a link to the folder with them. There are currently 2 examples: Pink Ripple, Flappy Bird.

Here's also a simple wave example built from ground up:
```rust
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
    let mut runtime = Runtime::new(false);
    runtime.run(&mut WaveProcess::new(Wave::new(-0.2, 0.4)));
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
```
# Info
This is a hobby project and as such I don't currently have any plans on supporting any other keyboards officially. But with a bit of tinkering it could maybe be done. But for a first time hobby project this is enough tinkering already for now.

Also this assumes my custom fn layout if you're going to use any key events. Check my config out: `045a0673980efcf95b5f5b6c959eb63b0f9f`
