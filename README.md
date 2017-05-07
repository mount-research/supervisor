# supervisor

Spawns a thread and which will restart the work fn when it produces a Result.

[Documentation](https://docs.rs/supervisor)

[Example usage](https://github.com/mount-research/supervisor/blob/master/examples/simple.rs):
```rust
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
