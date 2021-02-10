mod macros;
mod designators;
mod overload;

fn main() {
    macros::execute();
    designators::execute();
    overload::execute();
}
