mod threads;
mod testcase_mapreduce;

fn main() {
    threads::execute();
    testcase_mapreduce::execute();
}
