use std::{
    ffi::{c_float, c_int, c_uint, c_ushort},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI32, AtomicU8, AtomicU64, Ordering::Relaxed},
    },
    thread::{self, sleep},
    time::{Duration, Instant},
};

use crate::{
    key::Key,
    keyboard::{KeyboardMatrix, SCAN_CODE_LEN, SCAN_CODES, get_matrix, scan_code_to_matrix_pos},
};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use wooting_analog_wrapper::{
    ffi::{
        wooting_analog_initialise, wooting_analog_read_full_buffer,
        wooting_analog_set_keycode_mode, wooting_analog_uninitialise,
    },
    is_initialised,
};

#[derive(Clone, Debug, Default)]
pub struct KeyDelta {
    pub key: (u8, u8),
    pub scan_code: u16,
    pub distance: Arc<AtomicU8>,
    pub last_distance: Arc<AtomicU8>,
    pub delta: Arc<AtomicI32>,
    pub delta_average: Arc<AtomicI32>,
    /// In what distance should the key be considered pressed down
    /// (from 0 to 255, 255 being completely pressed down)
    pub key_press_distance: Arc<AtomicU8>,
    pub just_pressed: Arc<AtomicBool>,
}

impl KeyDelta {
    /// Whether the key was pressed just now.
    pub fn just_pressed(&self) -> bool {
        self.just_pressed.load(Relaxed)
    }

    /// Should be called periodically to clear just presses.
    /// This is automatically implemented in the runtime if you create one with analog support.
    pub fn just_pressed_consume(&self) {
        self.just_pressed.store(false, Relaxed);
    }
}

#[derive(Clone, Debug)]
pub struct DeltaWatcher {
    /// Per key delta data and related
    pub keys: [KeyDelta; SCAN_CODE_LEN],
    /// How much to wait in nanoseconds before scanning key states again
    pub scan_delay_ns: Arc<AtomicU64>,
    /// How many delta data should be in an average delta calculation
    pub deltas_in_average: Arc<AtomicU64>,
    /// To stop the delta watcher (true) or not (false).
    pub exit: Arc<AtomicBool>,
    /// Whether the watcher should be in power save mode or not
    pub idle: Arc<AtomicBool>,
    pub mat_keys: KeyboardMatrix,
}

impl DeltaWatcher {
    pub fn new(scan_delay: Duration, deltas_in_average: u64, key_press_distance: u8) -> Self {
        let s_ret = Self {
            keys: {
                SCAN_CODES.map(|scan_code| KeyDelta {
                    key: scan_code_to_matrix_pos(scan_code).expect("Dev error"),
                    scan_code,
                    delta: Arc::new(0.into()),
                    distance: Arc::new(0.into()),
                    last_distance: Arc::new(0.into()),
                    delta_average: Arc::new(0.into()),
                    just_pressed: Arc::new(false.into()),
                    key_press_distance: Arc::new(key_press_distance.into()),
                })
            },
            scan_delay_ns: Arc::new((scan_delay.as_nanos() as u64).into()),
            deltas_in_average: Arc::new(deltas_in_average.into()),
            exit: Arc::new(false.into()),
            mat_keys: get_matrix(),
            idle: Arc::new(false.into()),
        };

        let s = s_ret.clone();
        thread::spawn(move || {
            delta_watcher(s);
        });

        s_ret
    }

    /// Creates an uninitialized dummy delta watcher
    pub fn dummy() -> Self {
        Self {
            keys: SCAN_CODES.map(|_code| KeyDelta::default()),
            scan_delay_ns: Default::default(),
            deltas_in_average: Default::default(),
            exit: Default::default(),
            mat_keys: get_matrix(),
            idle: Arc::new(true.into()),
        }
    }

    /// Get all keys that are pressed down even slightly.
    pub fn get_pressed_keys(&self) -> Vec<&KeyDelta> {
        self.keys
            .iter()
            .filter(|key| key.distance.load(Relaxed) > 0)
            .collect()
    }

    /// Get all keys that are pressed down with their associated matrix key
    pub fn get_pressed_keys_mat_keys(&self) -> Vec<(&KeyDelta, &Key)> {
        self.get_pressed_keys()
            .iter()
            .map(|key| {
                (
                    *key,
                    self.mat_keys
                        .as_flattened()
                        .iter()
                        .find(|mat_key| mat_key.key == key.key)
                        .unwrap(),
                )
            })
            .collect()
    }

    /// Consumes every keypress in keys.
    /// (Calls this on every key)
    pub fn just_pressed_consume(&self) {
        for key in self.keys.iter() {
            key.just_pressed_consume();
        }
    }
}

impl Default for DeltaWatcher {
    fn default() -> Self {
        Self::new(Duration::from_millis(1), 50, 255 / 2)
    }
}

fn delta_watcher(s: DeltaWatcher) {
    unsafe { wooting_analog_initialise() };
    assert!(
        is_initialised(),
        "Wooting analog SDK is not initialised, no deltas will be provided"
    );
    unsafe { wooting_analog_set_keycode_mode(wooting_analog_wrapper::KeycodeType::ScanCode1) };

    let mut keys: [(KeyDelta, AllocRingBuffer<i32>); SCAN_CODE_LEN] = s.keys.clone().map(|key| {
        (
            key,
            AllocRingBuffer::new(s.deltas_in_average.load(Relaxed) as usize),
        )
    });

    let capacity: usize = SCAN_CODE_LEN;
    let mut code_buffer: Vec<c_ushort> = vec![0; capacity];
    let mut analog_buffer: Vec<c_float> = vec![0.0; capacity];

    let mut last = Instant::now();
    loop {
        let now = Instant::now();
        let delta = now.duration_since(last);

        let result: c_int = unsafe {
            wooting_analog_read_full_buffer(
                code_buffer.as_mut_ptr(),
                analog_buffer.as_mut_ptr(),
                capacity as c_uint,
            )
        };

        if result < 0 {
            println!("Error while reading values: {result}");
            break;
        }

        for (key, deltas_ring_buf) in keys.iter_mut() {
            let key_press_distance = key.key_press_distance.load(Relaxed);

            if deltas_ring_buf.len() != s.deltas_in_average.load(Relaxed) as usize {
                *deltas_ring_buf = AllocRingBuffer::new(s.deltas_in_average.load(Relaxed) as usize);
            }

            for i in 0..result as usize {
                let key_code = code_buffer[i];
                let distance = analog_buffer[i];
                if key.scan_code == key_code {
                    let distance = (distance * 255.0).round() as u8;
                    let last_distance = key.distance.load(Relaxed);

                    if distance >= key_press_distance && last_distance < key_press_distance {
                        key.just_pressed.store(true, Relaxed);
                    }

                    key.last_distance.store(last_distance, Relaxed);
                    key.distance.store(distance, Relaxed);

                    let dx = (last_distance as f64) - (distance as f64);
                    let dt = delta.as_secs_f64() * 10.0;

                    let mut v = dx / dt;
                    if v.is_nan() {
                        v = 0.0;
                    }

                    let v = -v.round() as i32;
                    key.delta.store(v, Relaxed);
                }
            }
            if code_buffer[0..result as usize].contains(&key.scan_code) {
                let v = key.delta.load(Relaxed);
                deltas_ring_buf.enqueue(v);
            } else {
                deltas_ring_buf.enqueue(0);
            }
            let delta_average =
                deltas_ring_buf.iter().sum::<i32>() / s.deltas_in_average.load(Relaxed) as i32;
            key.delta_average.store(delta_average, Relaxed);
        }

        sleep(Duration::from_nanos(s.scan_delay_ns.load(Relaxed)));
        if s.idle.load(Relaxed) {
            sleep(Duration::from_millis(200));
        }

        last = now;

        if s.exit.load(Relaxed) {
            break;
        }
    }

    if is_initialised() {
        unsafe { wooting_analog_uninitialise() };
    }

    println!("Exited delta watcher");
}

// /// Start a delta watcher which changes key delta based on keyboard key states
// /// Will not block and will spawn a new thread which does the job.
// /// Scan delay will determine how often the keys are scanned for changes.
// fn delta_watcher(
//     scan_delay: Duration,
//     deltas_in_average: u64,
//     key_press_distance: u8,
// ) -> [KeyDelta; SCAN_CODE_LEN] {
//     let keys_ret = SCAN_CODES.map(|scan_code| KeyDelta {
//         key: scan_code_to_matrix_pos(scan_code).expect("Dev error"),
//         scan_code,
//         delta: Arc::new(0.into()),
//         distance: Arc::new(0.into()),
//         last_distance: Arc::new(0.into()),
//         delta_average: Arc::new(0.into()),
//         key_press_distance: Arc::new(((key_press_distance * 255.0).round() as u8).into()),
//         just_pressed: Arc::new(false.into()),
//     });
//
//     let mut keys: [(KeyDelta, GrowableAllocRingBuffer<i32>); 133] = keys_ret.clone().map(|key| {
//         (
//             key,
//             GrowableAllocRingBuffer::with_capacity(deltas_in_average),
//         )
//     });
//     thread::spawn(move || {
//         unsafe { wooting_analog_initialise() };
//         assert!(
//             is_initialised(),
//             "Wooting analog SDK is not initialised, no deltas will be provided"
//         );
//         unsafe { wooting_analog_set_keycode_mode(wooting_analog_wrapper::KeycodeType::ScanCode1) };
//
//         let capacity: usize = SCAN_CODE_LEN;
//         let mut code_buffer: Vec<c_ushort> = vec![0; capacity];
//         let mut analog_buffer: Vec<c_float> = vec![0.0; capacity];
//
//         let mut last = Instant::now();
//         loop {
//             let now = Instant::now();
//             let delta = now.duration_since(last);
//
//             let result: c_int = unsafe {
//                 wooting_analog_read_full_buffer(
//                     code_buffer.as_mut_ptr(),
//                     analog_buffer.as_mut_ptr(),
//                     capacity as c_uint,
//                 )
//             };
//
//             if result < 0 {
//                 println!("Error while reading values: {result}");
//                 break;
//             }
//
//             for (key, deltas_ring_buf) in keys.iter_mut() {
//                 let key_press_distance = key.key_press_distance.load(Relaxed);
//                 for i in 0..result as usize {
//                     let key_code = code_buffer[i];
//                     let distance = analog_buffer[i];
//                     if key.scan_code == key_code {
//                         let distance = (distance * 255.0).round() as u8;
//                         let last_distance = key.distance.load(Relaxed);
//
//                         if distance >= key_press_distance && last_distance < key_press_distance {
//                             key.just_pressed.store(true, Relaxed);
//                         }
//
//                         key.last_distance.store(last_distance, Relaxed);
//                         key.distance.store(distance, Relaxed);
//
//                         let dx = (last_distance as f64) - (distance as f64);
//                         let dt = delta.as_secs_f64() * 10.0;
//
//                         let mut v = dx / dt;
//                         if v.is_nan() {
//                             v = 0.0;
//                         }
//
//                         // if v.round() != 0.0 {
//                         //     println!(
//                         //         "({last_distance}-{distance})/{dt:.2} = {dx:.2}/{dt:.2} = {v:.2}"
//                         //     );
//                         // }
//                         let v = -v.round() as i32;
//                         key.delta.store(v, Relaxed);
//                     }
//                 }
//                 if code_buffer[0..result as usize].contains(&key.scan_code) {
//                     let v = key.delta.load(Relaxed);
//                     deltas_ring_buf.enqueue(v);
//                 } else {
//                     deltas_ring_buf.enqueue(0);
//                 }
//                 let delta_average = deltas_ring_buf.iter().sum::<i32>() / deltas_in_average as i32;
//                 key.delta_average.store(delta_average, Relaxed);
//             }
//
//             sleep(scan_delay);
//
//             last = now;
//         }
//
//         if is_initialised() {
//             unsafe { wooting_analog_uninitialise() };
//         }
//     });
//
//     keys_ret
// }
