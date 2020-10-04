mod custom_types;
mod structs;
mod ct_enum;
mod enum_use;
mod c_like;
mod testcase_linked_list;

fn main() {
    custom_types::execute();
    structs::execute();
    ct_enum::execute();
    enum_use::execute();
    c_like::execute();
    testcase_linked_list::execute();
}
