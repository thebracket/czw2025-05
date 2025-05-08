//! This is my big public module

pub mod my_other_module;

pub use my_other_module::my_other_module;

/// Prints some text
/// 
/// ## Arguments
/// 
/// * `blah` - takes an integer
/// 
/// ## Safety
/// 
/// LEEROY JENKINS
/// 
/// ## Panics
/// 
/// Panics if blah is 0.
/// 
/// ## Example
/// 
/// ```rust
/// crate::my_module_print_it();
/// ```
pub fn print_it() {
    println!("Hello, world!");
}