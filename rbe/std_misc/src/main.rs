mod threads;
mod testcase_mapreduce;
mod channels;

fn main() {
    threads::execute();
    testcase_mapreduce::execute();
    channels::execute();
}
