mod cast;
mod literals;
mod inference;

fn main() {
    cast::execute();
    literals::execute();
    inference::execute();
}
