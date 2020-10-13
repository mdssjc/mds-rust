mod if_else;
mod fc_loop;
mod nested;
mod fc_return;

fn main() {
    if_else::execute();
    fc_loop::execute();
    nested::execute();
    fc_return::execute();
}
