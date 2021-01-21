mod visibility;
mod struct_visibility;
mod m_use;
mod m_super;

fn main() {
    visibility::execute();
    struct_visibility::execute();
    m_use::execute();
    m_super::execute();
}
