mod generics;
mod gen_fn;
mod generics_impl;
mod generics_trait;

fn main() {
    generics::execute();
    gen_fn::execute();
    generics_impl::execute();
    generics_trait::execute();
}
