// 1.2.2.1. Testcase: List

// Import the `fmt` module.
use std::fmt;

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

pub fn execute() {
    // Try `write!` to see if it errors. If it errors, return
    // the error. Otherwise continue.
    // write!(f, "{}", value)?;

    // try!(write!(f, "{}", value));

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
