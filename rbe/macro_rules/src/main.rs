mod macros;
mod designators;
mod overload;
mod repeat;
mod dry;
mod dsl;

fn main() {
    macros::execute();
    designators::execute();
    overload::execute();
    repeat::execute();
    dry::execute();
    dsl::execute();
}
