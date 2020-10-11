mod variable_bindings;
mod vb_mut;
mod scope;
mod declare;

fn main() {
    variable_bindings::execute();
    vb_mut::execute();
    scope::execute();
    declare::execute();
}
