//

// macro_rules! println2 {
//     ($($args:tt)*) => {
//         let message = format!("{}:{}: {}", file!(), line!(), format_args!($($args)*));
//         println!("{message}");
//     };
// }

use log2::init_logging;
use log2::{debug, error, info, trace, warn};

fn main() {
    let dir = "./logs";
    let name = "foo";
    init_logging(dir, name);

    // Generate some log messages to trigger rolling
    loop {
        error!("This is error message!");
        warn!("This is warn message!");
        info!("This is info message!");
        debug!("This is debug message!");
        trace!("This is trace message!");
    }
}
