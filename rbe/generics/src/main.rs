mod generics;
mod gen_fn;
mod generics_impl;
mod generics_trait;
mod bounds;
mod testcase_empty;
mod multi_bounds;

fn main() {
    generics::execute();
    gen_fn::execute();
    generics_impl::execute();
    generics_trait::execute();
    bounds::execute();
    testcase_empty::execute();
    multi_bounds::execute();
}
