use std::sync::Arc;

struct Timer {
    duration: f64,
    callback: Arc<fn() -> ()>,
}

impl Timer {
    fn new(duration: f64, callback: Arc<fn() -> ()>) -> Timer {
        Timer { duration, callback }
    }

    fn start(&self) {
        let dur = self.duration;
        let local_callback = self.callback.clone();
        let handler = std::thread::spawn(move || loop {
            let now = std::time::Instant::now();

            local_callback();

            let process_time = now.elapsed().as_secs_f64();

            if dur > process_time {
                let duration = std::time::Duration::from_secs_f64(dur - process_time);
                spin_sleep::sleep(duration);
            }
        });

        handler.join().unwrap();
    }
}

fn f() {
    println!("{:?}", std::time::Instant::now());
}

fn main() {
    let timer = Timer::new(1.0, Arc::new(f));
    timer.start();
}
