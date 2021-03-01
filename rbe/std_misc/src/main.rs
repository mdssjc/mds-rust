mod threads;
mod testcase_mapreduce;
mod channels;
mod path;

fn main() {
    threads::execute();
    testcase_mapreduce::execute();
    channels::execute();
    path::execute();
}
