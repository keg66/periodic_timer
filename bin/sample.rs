use periodic_timer::Timer;

fn timer_callback() {
    println!("[timer_callback] {:?}", std::time::Instant::now());
}

fn main() {
    let timer = Timer::new(0.5, timer_callback);
    timer.start();

    let timer2 = Timer::new(0.1, || {
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
