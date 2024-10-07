macro_rules! type_name2 {
    ($arg:expr) => {{
        fn type_name2<T: ?Sized>(_: &T) -> &str {
            std::any::type_name::<T>()
        }
        type_name2(&$arg)
    }};
}

//

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
