mod f_fn;
mod methods;
mod closures;
mod capture;
mod input_parameters;
mod anonymity;

fn main() {
    f_fn::execute();
    methods::execute();
    closures::execute();
    capture::execute();
    input_parameters::execute();
    anonymity::execute();
}
