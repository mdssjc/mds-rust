mod raii;
mod scope_move;
mod move_mut;
mod partial_move;

fn main() {
    raii::execute();
    scope_move::execute();
    move_mut::execute();
    partial_move::execute()
}
