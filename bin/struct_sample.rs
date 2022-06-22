use std::sync::Arc;
use std::sync::Mutex;

use simple_timer::Timer;

struct MyStruct {
    state: Arc<Mutex<f64>>,
    timer: Timer,
}

impl MyStruct {
    fn new() -> MyStruct {
        let state = Arc::new(Mutex::new(0.0));
        let local_state = state.clone();
        let callback = move || {
            let state = local_state.lock().unwrap();
            println!("current state: {}", *state);
        };

        let my_struct = MyStruct {
            state,
            timer: Timer::new(0.5, callback),
        };

        my_struct.timer.start();

        my_struct
    }

    fn set_state(&mut self, x: f64) {
        let mut state = self.state.lock().unwrap();
        *state = x;
    }
}

fn main() {
    let mut m = MyStruct::new();

    let dur = std::time::Duration::from_secs(1);
    let mut count = 0;
    loop {
        println!("[main] {:?}", std::time::Instant::now());

        println!("[main] set_state({})", count as f64);
        m.set_state(count as f64);

        spin_sleep::sleep(dur);

        count += 1;
    }
}
