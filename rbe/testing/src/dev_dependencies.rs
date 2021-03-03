// 21.4. Dev-dependencies

// externing crate for test-only use
#[cfg(test)]
use pretty_assertions::assert_eq;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}

pub fn execute() {}
