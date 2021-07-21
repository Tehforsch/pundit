use log::{Record, Level, Metadata};
use log::{LevelFilter, SetLoggerError};

static IDENTIFIER_LOGGER: ResultLogger = ResultLogger { add_identifier: true };
static NO_IDENTIFIER_LOGGER: ResultLogger = ResultLogger { add_identifier: false };

pub struct ResultLogger {
    pub add_identifier: bool,
}

impl log::Log for ResultLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        dbg!("ja hey");
        if self.enabled(record.metadata()) {
            match self.add_identifier {
                true => println!("\nOUTPUT{}", record.args()),
                false => println!("{}", record.args()),
            }
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(add_identifier: bool) -> core::result::Result<(), SetLoggerError> {
    let logger = match add_identifier {
        true => &IDENTIFIER_LOGGER,
        false => &NO_IDENTIFIER_LOGGER,
    };
    log::set_logger(logger)
        .map(|()| log::set_max_level(LevelFilter::Debug))
}
