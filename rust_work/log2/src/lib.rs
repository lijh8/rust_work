// logrotation with log4rs
// https://github.com/estk/log4rs/blob/main/examples/log_to_file_with_rolling.rs ,

// $ cd /path/to/mymodule  # package directory with cargo.toml
// $ cargo add log4rs log  # it will add dependencies and edit cargo.toml

use log::LevelFilter;
pub use log::{debug, error, info, trace, warn};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

pub fn init_logging(dir: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    const TRIGGER_FILE_SIZE: u64 = 1024u64 * 1024 * 1;
    const LOG_FILE_COUNT: u32 = 10;
    let file_path: &str = &format!("{dir}/{name}.log");

    // https://docs.rs/log4rs/*/log4rs/append/rolling_file/policy/compound/roll/fixed_window/struct.FixedWindowRollerBuilder.html#method.build ,
    let archive_pattern: &str = &format!("{dir}/{name}.{{}}.log");

    // https://docs.rs/log4rs/latest/log4rs/encode/pattern/index.html ,
    let pattern = "{d} {l} {file}:{line} - {m}\n";

    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let roller = FixedWindowRoller::builder().build(archive_pattern, LOG_FILE_COUNT)?;
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html ,
    // Logging to log file. (with rolling)
    let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(file_path, Box::new(policy))?;

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build();

    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Trace)))
                .build("logfile", Box::new(logfile)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Trace)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )?;

    let _handle = log4rs::init_config(config)?;
    Ok(())
}

/*

// workspace/members: "log2"
// package/dependencies: log2 = {path = "../log2"}

$ cat main.rs

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

$

*/
