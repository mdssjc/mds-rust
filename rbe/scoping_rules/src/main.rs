mod raii;
mod scope_move;
mod move_mut;

fn main() {
    raii::execute();
    scope_move::execute();
    move_mut::execute();
}
