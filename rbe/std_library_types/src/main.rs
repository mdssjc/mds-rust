mod std_box;
mod vec;
mod str;
mod option;
mod result;
mod question_mark;
mod panic;
mod hash;
mod alt_key_types;

fn main() {
    std_box::execute();
    vec::execute();
    str::execute();
    option::execute();
    result::execute();
    question_mark::execute();
    panic::execute();
    hash::execute();
    alt_key_types::execute();
}
