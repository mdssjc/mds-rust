mod rbe_trait;
mod derive;
mod trait_dyn;
mod ops;
mod drop;
mod iter;
mod impl_trait;

fn main() {
    rbe_trait::execute();
    derive::execute();
    trait_dyn::execute();
    ops::execute();
    drop::execute();
    iter::execute();
    impl_trait::execute();
}
