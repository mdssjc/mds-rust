mod f_fn;
mod methods;
mod closures;
mod capture;
mod input_parameters;
mod anonymity;
mod input_functions;
mod output_parameters;
mod iter_any;
mod iter_find;
mod hof;

fn main() {
    f_fn::execute();
    methods::execute();
    closures::execute();
    capture::execute();
    input_parameters::execute();
    anonymity::execute();
    input_functions::execute();
    output_parameters::execute();
    iter_any::execute();
    iter_find::execute();
    hof::execute();
}
