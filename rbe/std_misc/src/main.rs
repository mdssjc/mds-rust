mod threads;
mod testcase_mapreduce;
mod channels;
mod path;
mod open;
mod create;
mod read_lines;
mod process;
mod pipe;
mod wait;

fn main() {
    threads::execute();
    testcase_mapreduce::execute();
    channels::execute();
    path::execute();
    open::execute();
    create::execute();
    read_lines::execute();
    process::execute();
    pipe::execute();
    wait::execute();
}
