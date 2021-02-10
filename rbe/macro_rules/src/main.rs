mod macros;
mod designators;
mod overload;
mod repeat;
mod dry;

fn main() {
    macros::execute();
    designators::execute();
    overload::execute();
    repeat::execute();
    dry::execute();
}
