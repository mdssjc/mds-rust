mod if_else;
mod fc_loop;
mod nested;
mod fc_return;
mod fc_while;
mod fc_for;
mod fc_match;
mod destructure_tuple;
mod destructure_enum;
mod destructure_pointers;
mod destructure_structures;

fn main() {
    if_else::execute();
    fc_loop::execute();
    nested::execute();
    fc_return::execute();
    fc_while::execute();
    fc_for::execute();
    fc_match::execute();
    destructure_tuple::execute();
    destructure_enum::execute();
    destructure_pointers::execute();
    destructure_structures::execute();
}
