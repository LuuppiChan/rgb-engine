use std::{sync::atomic::Ordering::Relaxed, thread::sleep, time::Duration};

use lerp::Lerp;
use palette::Srgb;
use rgb_engine::{
    Effect,
    effects::{
        Ripple,
        analog::{Area, KeyFilter, LocalPressBrightness},
        perlin::{Direction, PerlinWave},
    },
    keyboard::{DeltaWatcher, KeyDelta, get_matrix, matrix::FN},
    runtime::{ColorBlendTypes, Key, Process, Runtime},
};
use tween::{Linear, Tweener};

const MAX_REVEALED: f64 = 5.0;
const FN_FADE_IN_TIME: f64 = 0.200;
const RIPPLE_LAYER: i32 = 100;
const BETWEEN_MAIN_FN: f64 = 0.5;
const ANIMATION_SMOOTHENING: f64 = 1.5;

fn main() {
    Runtime::new(true).run(&mut PinkRipple::default());
}

pub struct PinkRipple {
    pub background: PerlinWave,
    pub ripple: Ripple,
    pub reveal: Area,
    pub brightness: LocalPressBrightness,
    current_revealed: f64,
    fn_key: (Key, KeyDelta),
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
            reveal: Area::new(DeltaWatcher::dummy(), MAX_REVEALED, 1.0),
            current_revealed: 0.0,
            fn_key: (Key::default(), KeyDelta::default()),
            brightness: LocalPressBrightness::new(DeltaWatcher::dummy(), MAX_REVEALED, false),
        }
    }
}

impl Process for PinkRipple {
    type Owner = Runtime<Self>;

    fn init(&mut self, runtime: &mut Self::Owner) {
        let delta_watcher = runtime
            .delta_watcher
            .clone()
            .expect("Please enable delta_watcher");
        self.ripple.delta_watcher = delta_watcher.clone();
        self.reveal.delta_watcher = delta_watcher.clone();
        self.reveal.filter = KeyFilter::Included(vec![(FN)]);
        self.brightness.delta_watcher = delta_watcher.clone();
        self.brightness.filter = KeyFilter::Included(vec![FN]);

        let mut layer = get_matrix();
        for ele in layer.as_flattened_mut() {
            ele.color_blend_type = ColorBlendTypes::Mult;
        }
        runtime.create_layer(RIPPLE_LAYER, layer);
        runtime.create_layer(0, get_matrix());
        let fn_layer = {
            let red = Srgb::new(1.0, 0.0, 0.0);
            let green = Srgb::new(0.0, 1.0, 0.0);
            let blue = Srgb::new(0.0, 0.0, 1.0);
            let cyan = Srgb::new(0.0, 1.0, 1.0);
            let pink = Srgb::new(1.0, 0.0, 1.0);
            let yellow = Srgb::new(1.0, 1.0, 0.0);
            let white = Srgb::new(1.0, 1.0, 1.0);

            let mut layer = get_matrix();
            for ele in layer.as_flattened_mut() {
                ele.color_blend_type = ColorBlendTypes::Nothing;
            }
            layer[1][0].color = blue;
            for ele in layer[1][1..=12].iter_mut() {
                ele.color = green;
            }
            layer[1][13].color = red;

            layer[2][0].color = cyan;
            layer[2][1].color = red;
            layer[2][2].color = white;
            layer[2][3].color = green;
            layer[2][8].color = yellow;
            layer[2][9].color = pink;
            for ele in layer[2][10..=12].iter_mut() {
                ele.color = red;
            }

            layer[3][0].color = red;
            for ele in layer[3][1..=3].iter_mut() {
                ele.color = white;
            }
            for ele in layer[3][7..=9].iter_mut() {
                ele.color = yellow;
            }
            layer[3][10].color = white * 0.1;
            layer[3][11].color = white;
            layer[3][12].color = cyan;
            layer[3][13].color = green;

            layer[4][0].color = pink;
            layer[4][8].color = yellow;
            layer[4][9].color = red;
            layer[4][10].color = green;
            layer[4][11].color = white;
            layer[4][13].color = cyan;

            layer[5][0].color = pink;
            layer[5][1].color = white;
            for ele in layer[5][2..=9].iter_mut() {
                ele.color = pink;
            }
            for ele in layer[5][10..=12].iter_mut() {
                ele.color = white;
            }
            layer[5][13].color = cyan;

            layer
        };
        runtime.create_layer(200, fn_layer);
        let mut layer = get_matrix();
        for ele in layer.as_flattened_mut() {
            ele.color_blend_type = ColorBlendTypes::Sub;
        }
        runtime.create_layer(150, layer);

        runtime.create_layer(-100, fn_layer);
        self.fn_key = (
            fn_layer[5][13],
            delta_watcher
                .keys
                .iter()
                .find(|dkey| dkey.key == fn_layer[5][13].key)
                .unwrap()
                .clone(),
        );

        runtime.create_timer(Duration::from_millis(50), false, move |runtime, process| {
            let elapsed = runtime.start.elapsed().as_secs_f64();
            for key in runtime.get_layer(0).as_flattened_mut() {
                key.color = process.background.color(elapsed, key.pos_norm_aspect);
            }

            for key in runtime.get_layer(RIPPLE_LAYER).as_flattened_mut() {
                key.color = process.ripple.color(elapsed, key.pos_norm_aspect);
            }

            for i in 0..runtime.get_layer(200).as_flattened().len() {
                let distance_to_fn = runtime.get_layer(200).as_flattened_mut()[i]
                    .pos_norm_aspect
                    .metric_distance(&process.fn_key.0.pos_norm_aspect);
                if distance_to_fn < process.current_revealed * MAX_REVEALED {
                    if distance_to_fn > process.current_revealed * MAX_REVEALED - BETWEEN_MAIN_FN {
                        runtime.get_layer(200).as_flattened_mut()[i].color =
                            Srgb::new(0.0, 0.0, 0.0);
                        runtime.create_tween(
                            Tweener::new(0.0, 1.0, FN_FADE_IN_TIME, Box::new(Linear)),
                            move |r, _p, v| {
                                let other = r.get_layer(-100).as_flattened_mut()[i];
                                let key = &mut r.get_layer(200).as_flattened_mut()[i];

                                key.color = Srgb::new(0.0, 0.0, 0.0).lerp(other.color, v);

                                key.color_blend_type == ColorBlendTypes::Mask
                            },
                            |_r, _p, _v| {},
                        );
                    }
                    runtime.get_layer(200).as_flattened_mut()[i].color_blend_type =
                        ColorBlendTypes::Mask;
                } else {
                    runtime.get_layer(200).as_flattened_mut()[i].color_blend_type =
                        ColorBlendTypes::Nothing;
                }
            }

            let delta = runtime.delta.as_secs_f64();
            for key in runtime.get_layer(150).as_flattened_mut() {
                key.color = key.color.lerp(
                    process.brightness.color(0.0, key.pos_norm_aspect),
                    delta * 10.0,
                );
            }

            runtime.update_keyboard();
            true
        });
    }

    fn process(&mut self, runtime: &mut Self::Owner, delta: Duration) {
        sleep(Duration::from_millis(10).saturating_sub(delta));

        self.current_revealed = self.current_revealed.lerp(
            self.fn_key.1.distance.load(Relaxed) as f64 / 255.0,
            delta.as_secs_f64() * ANIMATION_SMOOTHENING,
        ) - 0.001;
        self.current_revealed = self.current_revealed.clamp(0.0, 1.0);

        self.ripple.update(runtime.start.elapsed().as_secs_f64());
    }
}
