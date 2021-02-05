// 15.4.3. Methods

struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one<'a>(&'a mut self) { self.0 += 1; }

    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

pub fn execute() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}
