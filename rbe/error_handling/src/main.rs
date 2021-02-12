mod panic;
mod option_unwrap;
mod question_mark;
mod map;
mod and_then;
mod result;
mod result_map;
mod result_alias;
mod early_returns;

fn main() {
    panic::execute();
    option_unwrap::execute();
    question_mark::execute();
    map::execute();
    and_then::execute();
    result::execute();
    result_map::execute();
    result_alias::execute();
    early_returns::execute();
}
