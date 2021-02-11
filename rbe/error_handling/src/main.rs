mod panic;
mod option_unwrap;
mod question_mark;
mod map;
mod and_then;

fn main() {
    panic::execute();
    option_unwrap::execute();
    question_mark::execute();
    map::execute();
    and_then::execute();
}
