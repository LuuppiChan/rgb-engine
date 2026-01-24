# rgb-engine
An RGB effect engine for Wooting 60HE.

Other Wooting keyboards may also work, but expect undefined behaviour. **Especially with DeltaWatcher!** (Analog and reactive effects such as ripple) DeltaWatcher has some hardcoded translation values from scancodes to positions based on my layout. (Please let me know if you know a better solution. Open an issue or something.)

# Installing
Add the following to your dependencies in Cargo.toml:
```toml
rgb-engine = { git = "https://github.com/LuuppiChan/rgb-engine.git" }
```

# Examples
Examples can be found inside the library and [here's](https://github.com/LuuppiChan/rgb-engine/tree/main/src/processes) also a link to the folder with them. There are currently 2 examples: [Pink Ripple](https://github.com/LuuppiChan/rgb-engine/blob/main/src/processes/pink_ripple.rs), [Flappy Bird](https://github.com/LuuppiChan/rgb-engine/blob/main/src/processes/flappy_bird/game.rs).

## Simple RGB Wave
Here's also a simple wave example built from ground up (should work on any Wooting keyboard):
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
## Pink Ripple (the example effect)
```rust
use rgb_engine::{examples::PinkRipple, runtime::Runtime};

fn main() {
    Runtime::new(true).run(&mut PinkRipple::default());
}
```

## Flappy Bird
- Space: Jump
- Esc: Reset/Exit

How to run:
```rust
use rgb_engine::examples::flappy_bird::run_flappy_bird;

fn main() {
    run_flappy_bird();
}
```


https://github.com/user-attachments/assets/3102dd82-1323-48ed-8640-8149546beb9e

# Info
This is a hobby project and as such I don't currently have any plans on supporting any other keyboards officially. However with a bit of tinkering it could likely be done (And some effects should work on them as of now). But for a first time hobby project this is enough tinkering for me for now.

Also this assumes my custom fn layout if you're going to use any key events. Check my config out: `045a0673980efcf95b5f5b6c959eb63b0f9f`

Lastly I must mention that I love Rust.
