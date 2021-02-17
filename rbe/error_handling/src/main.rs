mod panic;
mod option_unwrap;
mod question_mark;
mod map;
mod and_then;
mod result;
mod result_map;
mod result_alias;
mod early_returns;
mod enter_question_mark;
mod multiple_error_types;
mod option_result;
mod define_error_type;
mod boxing_errors;
mod reenter_question_mark;
mod wrap_error;
mod iter_result;

fn main() {
    panic::execute();
    option_unwrap::execute();
    question_mark::execute();
    map::execute();
    and_then::execute();
    result::execute();
    result_map::execute();
    result_alias::execute();
    early_returns::execute();
    enter_question_mark::execute();
    multiple_error_types::execute();
    option_result::execute();
    define_error_type::execute();
    boxing_errors::execute();
    reenter_question_mark::execute();
    wrap_error::execute();
    iter_result::execute();
}
