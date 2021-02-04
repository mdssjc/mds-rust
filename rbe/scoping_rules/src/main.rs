mod raii;
mod scope_move;
mod move_mut;
mod partial_move;
mod borrow;

fn main() {
    raii::execute();
    scope_move::execute();
    move_mut::execute();
    partial_move::execute();
    borrow::execute();
}
