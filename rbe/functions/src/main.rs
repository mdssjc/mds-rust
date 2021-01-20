mod f_fn;
mod methods;
mod closures;
mod capture;
mod input_parameters;
mod anonymity;
mod input_functions;
mod output_parameters;

fn main() {
    f_fn::execute();
    methods::execute();
    closures::execute();
    capture::execute();
    input_parameters::execute();
    anonymity::execute();
    input_functions::execute();
    output_parameters::execute();
}
