mod raii;
mod scope_move;
mod move_mut;
mod partial_move;
mod borrow;
mod borrow_mut;
mod alias;
mod borrow_ref;
mod lifetime;
mod explicit;

fn main() {
    raii::execute();
    scope_move::execute();
    move_mut::execute();
    partial_move::execute();
    borrow::execute();
    borrow_mut::execute();
    alias::execute();
    borrow_ref::execute();
    lifetime::execute();
    explicit::execute();
}
