use crate::{translate, Level, Locale, Logger};

pub struct SimpleLogger {
    level: Level,
}

impl Logger for SimpleLogger {
    fn new() -> Self {
        SimpleLogger {
            level: Level::DEBUG,
        }
    }

    fn initialize(&mut self, log_level: Level) {
        self.level = log_level
    }

    fn get_log_level(&self) -> Level {
        self.level
    }

    fn log(&self, level: Level, message: String) {
        println!(
            "simple logger Level: {} Message: {}",
            translate(Locale::th_TH, level),
            message,
        );
    }
}
