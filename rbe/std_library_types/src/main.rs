mod std_box;
mod vec;
mod str;
mod option;
mod result;
mod question_mark;
mod panic;

fn main() {
    std_box::execute();
    vec::execute();
    str::execute();
    option::execute();
    result::execute();
    question_mark::execute();
    panic::execute();
}
