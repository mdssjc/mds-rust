mod cast;
mod literals;
mod inference;
mod alias;

fn main() {
    cast::execute();
    literals::execute();
    inference::execute();
    alias::execute();
}
