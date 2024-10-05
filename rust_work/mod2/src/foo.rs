//[ example 1:
pub mod bar; // for sub-directory src/foo/bar.rs
use crate::bar as bar1; // src/bar.rs imported in main.rs
pub fn foo() {
    println!("foo");
    bar::bar(); // src/foo/bar.rs
    bar1::bar(); // src/bar.rs
}
//] example 1:

/*
//[ example 2:
pub mod bar;
pub fn foo() {
    println!("foo");
    crate::foo::bar::bar();
    crate::bar::bar();
}
//] example 2:
*/
