use simple_timer::Timer;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    let time = std::time::Instant::now();
    let prev = AtomicU64::new(time.elapsed().as_micros() as u64);
    let timer = Timer::new(0.1, move || {
        let time = time.elapsed().as_micros() as u64;

        println!("interval: {} us", time - prev.load(Ordering::SeqCst));
        prev.store(time, Ordering::SeqCst);
    });

    timer.start();

    loop {}
}
