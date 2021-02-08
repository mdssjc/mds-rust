mod rbe_trait;
mod derive;
mod trait_dyn;

fn main() {
    rbe_trait::execute();
    derive::execute();
    trait_dyn::execute();
}
