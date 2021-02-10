mod macros;
mod designators;
mod overload;
mod repeat;

fn main() {
    macros::execute();
    designators::execute();
    overload::execute();
    repeat::execute();
}
