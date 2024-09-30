//
// https://github.com/estk/log4rs/blob/main/examples/log_to_file_with_rolling.rs ,

pub use log::{debug, error, info, trace, warn};
use log::{LevelFilter, SetLoggerError};
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

pub fn init_logging(dir: &str, name: &str) -> Result<(), SetLoggerError> {
    const TRIGGER_FILE_SIZE: u64 = 1024u64 * 1024 * 1;
    const LOG_FILE_COUNT: u32 = 10;
    let file_path: &str = &format!("{dir}/{name}.log");

    // https://docs.rs/log4rs/*/log4rs/append/rolling_file/policy/compound/roll/fixed_window/struct.FixedWindowRollerBuilder.html#method.build ,
    let archive_pattern: &str = &format!("{dir}/{name}.{{}}.log");

    // https://docs.rs/log4rs/latest/log4rs/encode/pattern/index.html ,
    let pattern = "{d} {l} {file}:{line} - {m}\n";

    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let roller = FixedWindowRoller::builder()
        .build(archive_pattern, LOG_FILE_COUNT)
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html ,
    // Logging to log file. (with rolling)
    let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(file_path, Box::new(policy))
        .unwrap();

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
        )
        .unwrap();

    let _handle = log4rs::init_config(config)?;
    Ok(())
}

//

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
