mod variable_bindings;
mod vb_mut;
mod scope;
mod declare;
mod freeze;

fn main() {
    variable_bindings::execute();
    vb_mut::execute();
    scope::execute();
    declare::execute();
    freeze::execute();
}
