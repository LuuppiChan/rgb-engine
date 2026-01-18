use std::time::Duration;

use lerp::Lerp;
use nalgebra::Vector2;
use palette::{Srgb, num::ClampAssign};
use rand::{Rng, rng, rngs::ThreadRng};
use tween::{Linear, Tweener};

use crate::{
    effect::Effect,
    effects::perlin::{Direction, PerlinWave},
    key::{self, ColorBlendTypes},
    keyboard::{
        DeltaWatcher, KeyboardMatrix, get_matrix,
        matrix::{self, ESC, SPACE, compute_bounds},
    },
    process::{Process, Runtime, StandardTweener},
    processes::flappy_bird::Bounds,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum GameState {
    MainMenu,
    Playing,
    Dead,
}

enum Axis {
    X,
    Y,
}

struct GameSpace {
    up_axis: Axis,
    right_axis: Axis,
}

struct GameConfig {
    gravity: f64,
    flap_strength: f64,
    pipe_speed: f64,
    pipe_gap_size: f64,
    spawn_interval: f64,
    pipe_position_multiplier: f64,
    pipe_start_offset: f64,
    death_offset_from_bottom: f64,
    bird_color: Srgb<f64>,
    bird_color_death_animation: Srgb<f64>,
    pipe_color: Srgb<f64>,
    bird_spawn_pos: Vector2<f64>,
    score_color: Srgb<f64>,
    pipe_spawn_y_offset: f64,
}

struct World {
    bird: Bird,
    pipes: Vec<Pipe>,
    score_blocks: Vec<Bounds>,
    bounds: Bounds,
}

struct Bird {
    bounds: Bounds,
    velocity: Vector2<f64>,
    current_color: Srgb<f64>,
    alive: bool,
}

struct Pipe {
    bounds: Bounds,
}

impl Pipe {
    fn new(size: Vector2<f64>) -> Self {
        Self {
            bounds: Bounds {
                position: Vector2::new(0.0, 0.0),
                size,
            },
        }
    }
}

/// Yes, the flappy bird game written with this runtime system for the keyboard.
/// Uses the runtime analog feature. Please enable it in the runtime.
pub struct FlappyBird {
    rng: ThreadRng,
    world: World,
    config: GameConfig,
    current_state: GameState,
    score: u32,
    delta_watcher: DeltaWatcher,
    grass: PerlinWave,
    sky: PerlinWave,
}

impl Default for FlappyBird {
    fn default() -> Self {
        Self::new()
    }
}

impl FlappyBird {
    /// Create a new flappy bird game instance.
    pub fn new() -> Self {
        Self {
            rng: rng(),
            world: World {
                bird: Bird {
                    bounds: Bounds {
                        position: Vector2::new(-5.8, 0.0),
                        size: Vector2::new(0.4, 0.4),
                    },
                    velocity: Vector2::new(0.0, 0.0),
                    alive: true,
                    current_color: Srgb::new(0.0, 0.0, 0.0),
                },
                pipes: Vec::new(),
                bounds: compute_bounds(&get_matrix()),
                score_blocks: Vec::new(),
            },
            config: GameConfig {
                gravity: -2.0,
                flap_strength: 1.5,
                pipe_speed: 0.3,
                pipe_gap_size: 1.7,
                spawn_interval: 3.0,
                pipe_position_multiplier: 1.0,
                pipe_start_offset: 2.0,
                death_offset_from_bottom: 0.4,
                bird_color: Srgb::new(1.0, 1.0, 1.0),
                bird_color_death_animation: Srgb::new(1.0, 0.0, 0.0),
                pipe_color: Srgb::new(0.0, 1.0, 0.0),
                bird_spawn_pos: Vector2::new(-0.4, 0.0),
                score_color: Srgb::new(1.0, 1.0, 1.0),
                pipe_spawn_y_offset: -0.4,
            },
            current_state: GameState::MainMenu,
            delta_watcher: DeltaWatcher::dummy(),
            grass: PerlinWave::new(0, 0.1, 2.0),
            sky: PerlinWave::new(1, 0.05, 1.0),
            score: 0,
        }
    }

    fn render_bird(&self, layer: &mut KeyboardMatrix) {
        for key in layer.as_flattened_mut() {
            key.color = Srgb::new(0.0, 0.0, 0.0);
            key.color_blend_type = ColorBlendTypes::Nothing;
            let bird = &self.world.bird;

            if bird
                .bounds
                .contains(key.pos_norm.rotate(RotateDirection::AntiClockWise))
            {
                key.color = self.world.bird.current_color;
                key.color_blend_type = ColorBlendTypes::Mask;
            }
        }
    }

    fn render_pipes(&self, layer: &mut KeyboardMatrix) {
        for key in layer.as_flattened_mut() {
            key.color = Srgb::new(0.0, 0.0, 0.0);
            key.color_blend_type = ColorBlendTypes::Nothing;
            for pipe in self.world.pipes.iter() {
                if pipe
                    .bounds
                    .contains(key.pos_norm.rotate(RotateDirection::AntiClockWise))
                {
                    key.color = self.config.pipe_color;
                    key.color_blend_type = ColorBlendTypes::Mask;
                }
            }
        }
    }

    fn spawn_pipe(&mut self) {
        let offset = self.rng.random::<f64>() * self.config.pipe_position_multiplier - 1.0;
        let mut top = Pipe::new(Vector2::new(0.3, 10.0));
        top.bounds.position += Vector2::new(
            self.config.pipe_start_offset,
            self.config.pipe_gap_size + offset + self.config.pipe_spawn_y_offset,
        );
        let mut bot = Pipe::new(Vector2::new(0.3, 10.0));
        bot.bounds.position += Vector2::new(
            self.config.pipe_start_offset,
            offset - top.bounds.size.y + self.config.pipe_spawn_y_offset,
        );
        self.world.pipes.push(top);
        self.world.pipes.push(bot);

        let score_block = Bounds {
            position: Vector2::new(self.config.pipe_start_offset + 0.4, -30.0),
            size: Vector2::new(0.3, 100.0),
        };
        self.world.score_blocks.push(score_block);
    }

    fn flap(&mut self) {
        self.world.bird.velocity.y = -self.config.flap_strength;
    }

    fn apply_bird_velocity(&mut self, delta: f64) {
        self.world.bird.bounds.position += self.world.bird.velocity * delta;
    }

    fn reset(&mut self) {
        self.world.pipes.clear();
        self.world.score_blocks.clear();
        self.score = 0;
        self.world.bird.bounds.position = self.config.bird_spawn_pos;
        self.world.bird.velocity = Vector2::new(0.0, 0.0);
        self.world.bird.current_color = self.config.bird_color;
        self.current_state = GameState::MainMenu;
    }

    fn score(&mut self, layer: &mut KeyboardMatrix) {
        self.score += 1;
        self.render_score(layer);
    }

    fn render_score(&mut self, layer: &mut KeyboardMatrix) {
        let digits: Vec<char> = self.score.to_string().chars().collect();
        for key in layer.as_flattened_mut() {
            key.color = Srgb::new(0.0, 0.0, 0.0);
            key.color_blend_type = ColorBlendTypes::Nothing;
        }
        for digit in digits {
            let position = match digit {
                '0' => &mut layer[0][10],
                '1' => &mut layer[0][1],
                '2' => &mut layer[0][2],
                '3' => &mut layer[0][3],
                '4' => &mut layer[0][4],
                '5' => &mut layer[0][5],
                '6' => &mut layer[0][6],
                '7' => &mut layer[0][7],
                '8' => &mut layer[0][8],
                '9' => &mut layer[0][9],
                _ => unreachable!(),
            };
            position.color = self.config.score_color;
            position.color_blend_type = ColorBlendTypes::Mask;
        }
    }
}

impl Process for FlappyBird {
    type Owner = Runtime<Self>;

    fn init(&mut self, runtime: &mut Self::Owner) {
        println!("Starting game...");
        // println!(
        //     "World bounds: \nTop left: {}\nBottom right: {}",
        //     self.world.bounds.position,
        //     self.world.bounds.position + self.world.bounds.size
        // );
        self.delta_watcher = runtime.delta_watcher.clone().unwrap();
        self.world.bird.bounds.position = self.config.bird_spawn_pos;
        self.world.bird.current_color = self.config.bird_color;
        self.grass.hue_offset = 100.0;
        self.grass.hue_range = 30.0;
        self.grass.direction = Direction::Vertical;
        self.sky.hue_offset = 180.0;
        self.sky.hue_range = 40.0;
        self.sky.direction = Direction::Vertical;
        runtime.create_layer(2, get_matrix());
        runtime.create_layer(1, get_matrix());
        runtime.create_layer(0, get_matrix());
        runtime.create_layer(-1, get_matrix());
        runtime.create_layer(-2, get_matrix());

        runtime.create_timer(
            Duration::from_secs_f64(self.config.spawn_interval),
            false,
            move |_runtime, process| {
                if process.current_state == GameState::Playing {
                    process.spawn_pipe();
                }
                true
            },
        );

        runtime.create_timer(
            Duration::from_millis(100),
            false,
            move |runtime, process| {
                let elapsed = runtime.start.elapsed().as_secs_f64();
                for key in runtime.get_layer(-2).as_flattened_mut() {
                    if key.pos_norm.x < -1.2 {
                        key.color = process.grass.color(elapsed, key.pos_norm) * 0.3;
                    }
                }
                for key in runtime.get_layer(-1).as_flattened_mut() {
                    if key.pos_norm.x >= -1.2 {
                        key.color = process.sky.color(elapsed, key.pos_norm) * 0.05;
                    }
                }
                true
            },
        );
    }

    fn process(&mut self, runtime: &mut Self::Owner, delta: Duration) {
        runtime.update_keyboard();
        let just_jumped = self
            .delta_watcher
            .keys
            .iter()
            .find(|key| key.key == SPACE)
            .unwrap()
            .just_pressed();
        let esc_pressed = self
            .delta_watcher
            .keys
            .iter()
            .find(|key| key.key == ESC)
            .unwrap()
            .just_pressed();

        match self.current_state {
            GameState::MainMenu => {
                if just_jumped {
                    self.current_state = GameState::Playing;
                    runtime
                        .get_layer(2)
                        .as_flattened_mut()
                        .iter_mut()
                        .for_each(|key| {
                            key.color_blend_type = ColorBlendTypes::Nothing;
                            if key.key == matrix::ZERO {
                                key.color = self.config.score_color;
                                key.color_blend_type = ColorBlendTypes::Mask;
                            }
                        });
                    self.flap();
                } else if esc_pressed {
                    runtime.exit();
                }
            }
            GameState::Playing => {
                self.world.bird.velocity.y -= self.config.gravity * delta.as_secs_f64();

                if -self.world.bird.bounds.position.y
                    <= self.world.bounds.position.x + self.config.death_offset_from_bottom
                {
                    println!("You flew too low!");
                    self.current_state = GameState::Dead;
                    self.world.bird.current_color = self.config.bird_color_death_animation;
                }

                if just_jumped {
                    self.flap();
                }

                for pipe in self.world.pipes.iter_mut() {
                    if pipe.bounds.intersects(&self.world.bird.bounds) {
                        println!("You hit a pipe!");
                        self.current_state = GameState::Dead;
                        self.world.bird.current_color = self.config.bird_color_death_animation;
                    }

                    pipe.bounds.position.x -= self.config.pipe_speed * delta.as_secs_f64();
                }

                if !self.world.score_blocks.is_empty() {
                    for i in 0..self.world.score_blocks.len() {
                        self.world.score_blocks[i].position.x -=
                            self.config.pipe_speed * delta.as_secs_f64();

                        if self.world.score_blocks[i].intersects(&self.world.bird.bounds) {
                            // println!("Scored");
                            self.score(runtime.get_layer(2));
                        }
                    }

                    self.world
                        .score_blocks
                        .retain(|score_block| !score_block.intersects(&self.world.bird.bounds));
                }

                if esc_pressed {
                    self.reset();
                }
            }
            GameState::Dead => {
                self.world.bird.velocity.y -= self.config.gravity * delta.as_secs_f64();
                if esc_pressed {
                    self.reset();
                }
            }
        }

        self.world
            .pipes
            .retain(|pipe| pipe.bounds.position.x > -self.config.pipe_start_offset);

        if -self.world.bird.bounds.position.y
            <= self.world.bounds.position.x + self.config.death_offset_from_bottom
        {
            self.world.bird.velocity.y = 0.0;
        }
        // println!(
        //     "Bird: {} ground: {}",
        //     self.world.bird.bounds.position.y,
        //     -self.world.bounds.position.x + self.config.death_offset_from_bottom
        // );

        self.apply_bird_velocity(delta.as_secs_f64());
        self.render_bird(runtime.get_layer(1));
        self.render_pipes(runtime.get_layer(0));
    }
}

trait Rotate {
    fn rotate(&self, direction: RotateDirection) -> Vector2<f64>;
}

impl Rotate for Vector2<f64> {
    fn rotate(&self, direction: RotateDirection) -> Vector2<f64> {
        match direction {
            RotateDirection::ClockWise => Vector2::new(self.y, self.x),
            RotateDirection::AntiClockWise => Vector2::new(self.y, -self.x),
        }
    }
}

enum RotateDirection {
    ClockWise,
    AntiClockWise,
}
