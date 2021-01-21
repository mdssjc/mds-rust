mod visibility;
mod struct_visibility;
mod m_use;

fn main() {
    visibility::execute();
    struct_visibility::execute();
    m_use::execute();
}
