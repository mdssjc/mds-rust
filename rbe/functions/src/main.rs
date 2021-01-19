mod f_fn;
mod methods;
mod closures;
mod capture;

fn main() {
    f_fn::execute();
    methods::execute();
    closures::execute();
    capture::execute();
}
