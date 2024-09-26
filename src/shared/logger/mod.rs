use crate::shared::error::{Error, Result};
use log4rs::{
    append::console::{ConsoleAppender, Target},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    init_config,
};

pub fn setup() -> Result<()> {
    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}{n}")))
        .target(Target::Stdout)
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .build(
            Root::builder()
                .appender("console")
                .build(log::LevelFilter::Debug),
        )
        .map_err(|e| {
            println!("{}", e);
            Error::Unknown
        })?;

    init_config(config).map_err(|e| {
        println!("{}", e);
        Error::Unknown
    })?;
    Ok(())
}
