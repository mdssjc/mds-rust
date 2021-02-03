mod raii;
mod scope_move;

fn main() {
    raii::execute();
    scope_move::execute();
}
