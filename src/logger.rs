use log::{Record, Level, Metadata};

pub struct ResultLogger {
    pub add_identifier: bool,
}

impl log::Log for ResultLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match self.add_identifier {
                true => println!("\nOUTPUT{}", record.args()),
                false => println!("{}", record.args()),
            }
        }
    }

    fn flush(&self) {}
}
