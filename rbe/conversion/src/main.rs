mod from_into;
mod try_from_try_into;
mod string;

fn main() {
    from_into::execute();
    try_from_try_into::execute();
    string::execute();
}
