mod visibility;
mod struct_visibility;
mod m_use;
mod m_super;
mod split;

fn main() {
    visibility::execute();
    struct_visibility::execute();
    m_use::execute();
    m_super::execute();
    split::execute();
}
