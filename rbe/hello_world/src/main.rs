mod hello;
mod comment;
mod print;
mod print_debug;
mod print_display;
mod testcase_list;
mod fmt;

fn main() {
    hello::execute();
    comment::execute();
    print::execute();
    print_debug::execute();
    print_display::execute();
    testcase_list::execute();
    fmt::execute();
}
