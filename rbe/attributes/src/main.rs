// 13.1. dead_code
// 13.2. Crates

// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "rary"]

fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

fn main() {
    used_function();
}
