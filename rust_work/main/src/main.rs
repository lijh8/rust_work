/*
mod bar; // pub required for access from outside the crate
mod foo;
use foo::bar as bar2; // sub-directory src/foo/bar.rs imported in foo.fs

fn main2() {
    println!("main");
    foo::foo();
    bar::bar();
    bar2::bar();
}

*/
use log2::init_logging;
use log2::{debug, error, info, trace, warn};

fn main() {
    let dir = "./logs";
    let name = "foo";
    let _ = match init_logging(dir, name) {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    };

    // Generate some log messages to trigger rolling
    loop {
        error!("This is error message!");
        warn!("This is warn message!");
        info!("This is info message!");
        debug!("This is debug message!");
        trace!("This is trace message!");
    }
}
