mod rbe_trait;
mod derive;
mod trait_dyn;
mod ops;
mod drop;

fn main() {
    rbe_trait::execute();
    derive::execute();
    trait_dyn::execute();
    ops::execute();
    drop::execute();
}
