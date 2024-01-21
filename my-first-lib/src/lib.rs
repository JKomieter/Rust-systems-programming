use std::process;
pub fn hello_from_lib(message: &str) {
    println!("Hello from lib: {}", message);
}
#! [Docs for integ-test-example crate]
//! Integration-test-example crate
//!
//! This is a library that contains functions related to
//! dealing with processes
//! , and makes these tasks more convenient.
/// This function gets the process id of the current
/// executable. It returns a non-zero number
/// ```
/// fn get_id() {
/// let x = integ_test_example::get_process_id();
/// println!("{}",x);
/// }
/// ```
pub fn get_process_id() -> u32 {
process::id()
}