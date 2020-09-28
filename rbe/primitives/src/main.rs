mod primitives;
mod literals;
mod tuples;
mod array;

fn main() {
    primitives::execute();
    literals::execute();
    tuples::execute();
    array::execute();
}
