#[macro_use] extern crate log;

mod supervise;

use std::process::Command;
use supervise::supervise;

fn main(){
    let args = std::env::args().nth(0).unwrap();

    supervise(move || -> Result<&'static str, &'static str> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(args.to_string())
            .output()
            .expect("failed to execute process");

        let result = std::str::from_utf8(&output.stdout).unwrap();

        info!("{}", result);

        Ok("test")
    });
}
