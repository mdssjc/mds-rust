// 20.5.2. Wait

use std::process::Command;

pub fn execute() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
