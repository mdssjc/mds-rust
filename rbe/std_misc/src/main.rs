mod threads;
mod testcase_mapreduce;
mod channels;
mod path;
mod open;
mod create;
mod read_lines;

fn main() {
    threads::execute();
    testcase_mapreduce::execute();
    channels::execute();
    path::execute();
    open::execute();
    create::execute();
    read_lines::execute();

}
