use std::thread;
use std::fmt::{ Display, Debug };

/// Spawns a thread and which will restart the work_fn when it produces a Result
pub fn supervise<F, O, E>(work_fn: F) 
    where F: Fn() -> Result<O, E> + 'static + Send, O: Debug + Display, E: Debug + Display {

    thread::spawn(move || {
        loop {
            let result = work_fn();

            match result {
                Ok(l) => info!("{}", l),
                Err(e) => error!("Error: {}", e),
            } 
        }
    });
}

#[test]
fn supervise_spawn() {
    supervise(move || -> Result<&'static str, &'static str> {
        for i in 0 .. 5 {
            if i == 4 {
                return Err("failed work reason");
            }
        }
        Ok("finished")
    });
}
