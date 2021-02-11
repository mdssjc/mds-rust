mod panic;
mod option_unwrap;
mod question_mark;

fn main() {
    panic::execute();
    option_unwrap::execute();
    question_mark::execute();
}
