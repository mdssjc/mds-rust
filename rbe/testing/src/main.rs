mod unit_testing;
mod doc_testing;
mod integration_testing;
mod dev_dependencies;

fn main() {
    unit_testing::execute();
    doc_testing::execute();
    integration_testing::execute();
    dev_dependencies::execute();
}
