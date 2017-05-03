#[macro_use] extern crate log;

use std::thread;

/// Spawns a thread and which will restart the work_fn when it produces a Result
pub fn supervise<F: 'static>(work_fn: F) where F: Fn() -> Result<&'static str, &'static str> + Send {
    thread::spawn(move || {
        loop {
            let result = work_fn();

            if let Err(e) = result {
                error!("Error: {}", e);
            }
        }
    });
}

#[test]
fn supervise_spawn() {
    supervise(move || {
        for i in 0 .. 5 {
            if i == 4 {
                return Err("failed work reason");
            }
        }
        Ok("finished")
    });
}
