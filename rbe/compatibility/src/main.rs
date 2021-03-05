// 23.1. Raw identifiers

// extern crate foo;
mod foo {
    pub fn r#try() {
        unimplemented!();
    }
}

fn main() {
    foo::r#try();
}
