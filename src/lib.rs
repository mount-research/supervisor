/*!

Spawns a thread and which will restart the work fn when it produces a Result.

[Example usage](https://github.com/mount-research/supervisor/blob/master/examples/simple.rs):

```no_run
extern crate supervisor;

fn main() {
    supervisor::supervise(move || -> Result<&'static str, &'static str> {
        for i in 0 .. 5 {
            println!("trying: {}", i);
            if i == 4 {
                return Err("failed work reason");
            }
        }
        Ok("finished")
    });

    // run an inf loop whilst supervised thread does work
    loop {
        std::thread::sleep(std::time::Duration::new(1u64, 0u32));
    }
}
```

*/

#![deny(missing_docs)]

#[macro_use]
extern crate log;

use std::fmt::{Debug, Display};
use std::thread;

/// Spawns a thread and which will restart the work_fn when it produces a Result
pub fn supervise<F, O, E>(work_fn: F)
where
    F: Fn() -> Result<O, E> + 'static + Send,
    O: Debug + Display,
    E: Debug + Display,
{
    thread::spawn(move || loop {
        let result = work_fn();

        match result {
            Ok(l) => info!("{}", l),
            Err(e) => error!("Error: {}", e),
        }
    });
}

#[test]
fn supervise_spawn() {
    supervise(move || -> Result<&'static str, &'static str> {
        for i in 0..5 {
            if i == 4 {
                return Err("failed work reason");
            }
        }
        Ok("finished")
    });
}

#[test]
fn supervise_multi_spawn() {
    supervise(move || -> Result<&'static str, &'static str> {
        for i in 0..5 {
            if i == 4 {
                return Err("failed work reason");
            }
        }
        Ok("finished")
    });
    supervise(move || -> Result<&'static str, &'static str> {
        for i in 5..10 {
            if i == 7 {
                return Err("failed work reason");
            }
        }
        Ok("finished")
    });
}
