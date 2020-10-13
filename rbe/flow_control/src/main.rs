mod if_else;
mod fc_loop;
mod nested;
mod fc_return;
mod fc_while;
mod fc_for;

fn main() {
    if_else::execute();
    fc_loop::execute();
    nested::execute();
    fc_return::execute();
    fc_while::execute();
    fc_for::execute();
}
