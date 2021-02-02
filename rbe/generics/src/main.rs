mod generics;
mod gen_fn;
mod generics_impl;
mod generics_trait;
mod bounds;
mod testcase_empty;
mod multi_bounds;
mod generics_where;
mod new_types;
mod the_problem;
mod types;
mod phantom;
mod testcase_units;

fn main() {
    generics::execute();
    gen_fn::execute();
    generics_impl::execute();
    generics_trait::execute();
    bounds::execute();
    testcase_empty::execute();
    multi_bounds::execute();
    generics_where::execute();
    new_types::execute();
    the_problem::execute();
    types::execute();
    phantom::execute();
    testcase_units::execute();
}
