// 11.2. Using a Library
// rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable

fn main() {
    lib::public_function();
    lib::indirect_access();
}
