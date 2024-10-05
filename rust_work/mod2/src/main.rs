mod bar; // pub required for access from outside the crate
mod foo;
use foo::bar as bar2; // sub-directory src/foo/bar.rs imported in foo.fs

fn main() {
    println!("main mod");
    foo::foo();
    bar::bar();
    bar2::bar();
}
