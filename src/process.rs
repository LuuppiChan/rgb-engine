#![allow(dead_code)]

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::atomic::Ordering::Relaxed,
    time::{Duration, Instant},
};

use tween::{Tween, Tweener};
use wooting_rgb::RgbKeyboard;

use crate::{
    key::ColorBlendTypes,
    keyboard::{DeltaWatcher, KeyboardMatrix, get_matrix},
    timer::Timer,
};

pub type StandardTweener = Tweener<f64, f64, Box<dyn Tween<f64>>>;
pub type TweenCallback<T> = Rc<dyn Fn(&mut Runtime<T>, &mut T, f64) -> bool>;
pub type TweenFinishedCallback<T> = Option<Rc<dyn Fn(&mut Runtime<T>, &mut T, f64)>>;
pub type StandardTweenerData<T> = (
    Rc<RefCell<StandardTweener>>,
    TweenCallback<T>,
    TweenFinishedCallback<T>,
);

pub struct Runtime<T: Process<Owner = Self>> {
    pub keyboard: RgbKeyboard,
    /// Since the runtime started
    pub start: Instant,
    /// Delta watcher you can copy for other things
    pub delta_watcher: Option<DeltaWatcher>,
    /// Delta since last frame.
    /// This is for when you cannot access it from process. Like in a timer or tweener.
    pub delta: Duration,
    exit: bool,
    tweeners: Vec<StandardTweenerData<T>>,
    timers: Vec<Rc<RefCell<Timer<T>>>>,
    effect_layers: HashMap<i32, KeyboardMatrix>,
    render_layer: KeyboardMatrix,
}

impl<T: Process<Owner = Self>> Runtime<T> {
    /// Create a new runtime
    pub fn new(analog: bool) -> Self {
        Self {
            delta_watcher: if analog {
                Some(DeltaWatcher::new(Duration::from_millis(1), 10, 255 / 2))
            } else {
                None
            },
            exit: false,
            tweeners: Vec::new(),
            timers: Vec::new(),
            start: Instant::now(),
            effect_layers: HashMap::new(),
            keyboard: RgbKeyboard,
            delta: Duration::ZERO,
            render_layer: get_matrix(),
        }
    }

    /// Create a layer you can use for effects.
    pub fn create_layer(&mut self, z_index: i32, layer: KeyboardMatrix) {
        self.effect_layers.insert(z_index, layer);
    }

    /// # Panics
    /// If the z_index does not exist.
    pub fn get_layer(&mut self, z_index: i32) -> &mut KeyboardMatrix {
        self.effect_layers.get_mut(&z_index).unwrap()
    }

    /// Run the process loop
    pub fn run(&mut self, process: &mut T) {
        assert!(
            wooting_rgb::is_wooting_keyboard_connected(),
            "Wooting keyboard not connected"
        );
        // reset state to default
        self.exit = false;
        self.effect_layers.clear();
        self.timers.clear();
        self.tweeners.clear();
        self.start = Instant::now();

        process.init(self);
        let mut last = Instant::now();
        loop {
            let now = Instant::now();
            let delta = now.duration_since(last);
            self.delta = delta;
            last = now;

            let mut tweeners = self.tweeners.clone();
            tweeners.retain(|(tweener, callback, finished)| {
                let value = tweener.borrow_mut().move_by(delta.as_secs_f64());
                let retain = callback(self, process, value);
                if let Some(finished) = finished
                    && tweener.borrow().is_finished()
                {
                    finished(self, process, value);
                }
                !tweener.borrow().is_finished() && retain
            });
            self.tweeners = tweeners;

            // for timer in self.timers.clone().iter() {
            //     let mut timer = timer.borrow_mut();
            //     if timer.is_finished() {
            //         timer.timeout(self, process);
            //     }
            // }

            let mut timers = self.timers.clone();
            timers.retain(|timer| {
                let mut timer = timer.borrow_mut();
                if timer.is_finished() {
                    timer.timeout(self, process);
                }

                !timer.is_finished() || !timer.one_shot || !timer.continue_running
            });
            self.timers = timers;

            process.process(self, delta);

            if let Some(delta_watcher) = &self.delta_watcher {
                delta_watcher.just_pressed_consume();
            }

            if self.exit {
                break;
            }
        }
    }

    /// Exit the process loop after this iteration.
    pub fn exit(&mut self) {
        if let Some(delta_watcher) = &self.delta_watcher {
            delta_watcher.exit.store(true, Relaxed);
        }

        self.exit = true;
    }

    /// Creates a tweener based on the given tweener.
    pub fn create_tween<
        U: Fn(&mut Runtime<T>, &mut T, f64) -> bool + 'static,
        W: Fn(&mut Runtime<T>, &mut T, f64) + 'static,
    >(
        &mut self,
        tweener: StandardTweener,
        callback: U,
        finished: W,
    ) {
        let tweener = Rc::new(RefCell::new(tweener));
        self.tweeners
            .push((tweener, Rc::new(callback), Some(Rc::new(finished))));
    }

    /// Creates a timer and starts it
    /// Calls it on the first process iteration that it's finished
    pub fn create_timer<U: Fn(&mut Runtime<T>, &mut T) -> bool + 'static>(
        &mut self,
        timeout: Duration,
        one_shot: bool,
        callback: U,
    ) {
        self.timers.push(Rc::new(RefCell::new(Timer::start(
            timeout,
            one_shot,
            Box::new(callback),
        ))));
    }

    /// Updates the keyboard rgb array.
    /// This is quite intensive due to keyboard communication. (around 16 ms)
    pub fn update_keyboard(&mut self) {
        let mut render = self.render_layer;

        let mut ks = self.effect_layers.keys().copied().collect::<Vec<_>>();
        ks.sort();

        {
            let render = render.as_flattened_mut();
            for k in ks {
                let layer = *self.get_layer(k);
                for (i, key) in layer.as_flattened().iter().enumerate() {
                    match key.color_blend_type {
                        ColorBlendTypes::Add => render[i].color += key.color,
                        ColorBlendTypes::Sub => render[i].color -= key.color,
                        ColorBlendTypes::Mult => {
                            render[i].color.red *= key.color.red;
                            render[i].color.green *= key.color.green;
                            render[i].color.blue *= key.color.blue;
                        }
                        ColorBlendTypes::Mask => render[i].color = key.color,
                        ColorBlendTypes::Nothing => (),
                        ColorBlendTypes::AlphaBlend(lower, upper) => {
                            render[i].color *= lower;
                            render[i].color += key.color * upper;
                        }
                    };
                }
            }
        }

        for key in render.as_flattened() {
            let (red, green, blue) = key.colors();
            self.keyboard.array_set_single(key.key, red, green, blue);
        }
        self.keyboard.array_update();
    }
}

pub trait Process {
    type Owner;

    /// Called once before the loop starts
    fn init(&mut self, runtime: &mut Self::Owner);
    /// Called every step of the loop
    fn process(&mut self, runtime: &mut Self::Owner, delta: Duration);
}
