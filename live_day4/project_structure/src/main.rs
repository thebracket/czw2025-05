mod my_module;
mod foobar;

fn main() {
    my_module::print_it();
    
    use my_module::print_it;
    use my_module::*;
    print_it();
    
    my_module::my_other_module();
    
    crate::foobar::foobar_fun()
}
