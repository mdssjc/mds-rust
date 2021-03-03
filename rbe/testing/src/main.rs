mod unit_testing;
mod doc_testing;
mod integration_testing;

fn main() {
    unit_testing::execute();
    doc_testing::execute();
    integration_testing::execute();
}
