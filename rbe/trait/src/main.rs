mod rbe_trait;
mod derive;
mod trait_dyn;
mod ops;
mod drop;
mod iter;
mod impl_trait;
mod clone;
mod supertraits;
mod disambiguating;

fn main() {
    rbe_trait::execute();
    derive::execute();
    trait_dyn::execute();
    ops::execute();
    drop::execute();
    iter::execute();
    impl_trait::execute();
    clone::execute();
    supertraits::execute();
    disambiguating::execute();
}
