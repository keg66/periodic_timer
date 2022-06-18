use std::{
    sync::{atomic::AtomicBool, atomic::Ordering, Arc},
    thread::JoinHandle,
};

struct Timer {
    duration: f64,
    callback: Arc<fn() -> ()>,
    handler: Option<JoinHandle<()>>,
    is_running: Arc<AtomicBool>,
}

impl Timer {
    fn new(duration: f64, callback: fn() -> ()) -> Timer {
        Timer {
            duration,
            callback: Arc::new(callback),
            handler: None,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    fn start(&mut self) {
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
                    let duration = std::time::Duration::from_secs_f64(dur - process_time);
                    spin_sleep::sleep(duration);
                }
            }
        }));
    }

    fn stop(&mut self) {
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

fn timer_callback() {
    println!("[timer_callback] {:?}", std::time::Instant::now());
}

fn main() {
    let mut timer = Timer::new(0.5, timer_callback);
    timer.start();

    let mut timer2 = Timer::new(0.1, || {
        println!("[timer_callback closure] {:?}", std::time::Instant::now());
    });

    let dur = std::time::Duration::from_secs(1);
    let mut count = 0;
    loop {
        println!("[main] {:?}", std::time::Instant::now());
        spin_sleep::sleep(dur);

        count += 1;
        if count == 5 {
            println!("stop timer");
            timer.stop();
        }
        if count == 10 {
            println!("start timer2");
            timer2.start();
        }
    }
}
