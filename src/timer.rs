use std::time::{Duration, Instant};

use crate::process::{Process, Runtime};

pub type TimerCallback<T> = Box<dyn Fn(&mut Runtime<T>, &mut T) -> bool>;

/// Simple timer class which calls the given callback once the timer has reached timeout.
/// Checked once per process loop.
pub struct Timer<T: Process<Owner = Runtime<T>>> {
    pub one_shot: bool,
    /// This is for the runtime to determine whether to drop this timer or keep it in the process
    /// loop.
    pub continue_running: bool,
    elapsed: Instant,
    timeout: Duration,
    callback: TimerCallback<T>,
}

impl<T: Process<Owner = Runtime<T>>> Timer<T> {
    pub fn start(timeout: Duration, one_shot: bool, callback: TimerCallback<T>) -> Self {
        Self {
            elapsed: Instant::now(),
            continue_running: true,
            timeout,
            one_shot,
            callback,
        }
    }

    /// Whether given time has elapsed
    pub fn is_finished(&self) -> bool {
        self.elapsed.elapsed() >= self.timeout
    }

    /// Run a timeout
    pub fn timeout(&mut self, runtime: &mut Runtime<T>, process: &mut T) {
        // reset timer
        if !self.one_shot {
            self.elapsed = Instant::now();
        }
        // call the callback
        self.continue_running = (self.callback)(runtime, process);
    }
}
