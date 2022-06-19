//! # Simple Timer
//!
//! `simple_timer` is a simple implementation of cyclic timer.
//!
//! # Usage
//!
//! ```
//! use simple_timer::Timer;
//!
//! // Prepare callback function to be periodically executed
//! let callback = move || println!("hello timer!");
//!
//! // Create new timer
//! let mut timer = Timer::new(1.0, callback);
//!
//! // Start timer
//! timer.start();
//! ```
//!

use std::{
    sync::{atomic::AtomicBool, atomic::Ordering, Arc},
    thread::JoinHandle,
};

/// Simple implementation of cyclic timer.
pub struct Timer {
    duration: f64,
    callback: Arc<dyn Fn() -> () + Send + Sync + 'static>,
    handler: Option<JoinHandle<()>>,
    is_running: Arc<AtomicBool>,
}

impl Timer {
    /// Create new Timer.
    ///
    /// * `duration` target duration \[sec\]
    /// * `callback` callback function to be periodically executed
    pub fn new<F: Fn() -> () + Send + Sync + 'static>(duration: f64, callback: F) -> Timer {
        Timer {
            duration,
            callback: Arc::<F>::new(callback),
            handler: None,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start Timer.
    pub fn start(&mut self) {
        let dur = self.duration;
        let local_callback = self.callback.clone();

        let local_is_running = self.is_running.clone();
        local_is_running.store(true, Ordering::SeqCst);

        self.handler = Some(std::thread::spawn(move || {
            while local_is_running.load(Ordering::SeqCst) {
                let now = std::time::Instant::now();

                local_callback();

                let process_time = now.elapsed().as_secs_f64();

                if dur > process_time {
                    spin_sleep::sleep(std::time::Duration::from_secs_f64(dur - process_time));
                }
            }
        }));
    }

    /// Stop Timer.
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);

        if let Some(handler) = self.handler.take() {
            handler.join().unwrap();
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.stop();
    }
}
