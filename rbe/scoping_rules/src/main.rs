mod raii;
mod scope_move;
mod move_mut;
mod partial_move;
mod borrow;
mod borrow_mut;
mod alias;

fn main() {
    raii::execute();
    scope_move::execute();
    move_mut::execute();
    partial_move::execute();
    borrow::execute();
    borrow_mut::execute();
    alias::execute();
}
