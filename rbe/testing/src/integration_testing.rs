// 21.3. Integration testing

// Define this in a crate called `adder`.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    setup();
    assert_eq!(add(3, 2), 5);
}

pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
}

pub fn execute() {}
