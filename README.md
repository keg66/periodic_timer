Periodic Timer
==

`periodic_timer` is a simple Rust implementation of periodic timer.

# Usage

```rust
use periodic_timer::Timer;

// Prepare callback function to be periodically executed
let callback = || println!("hello timer!");

// Create new timer
let timer = Timer::new(1.0, callback);

// Start timer
timer.start();
```