#![allow(non_camel_case_types)]
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::fmt;

// /* These represent essembly's logging levels. Logging levels
// ///are inclusive of the other levels 'above' them, so `WARN`
// ///includes `INFO` and `ERROR`
// ///TRACE: Absolutely everything, including some system calls.
// ///DEBUG: Normally used for trouble-shooting.
// ///WARN: Indicates some state that may be problematic.
// ///INFO: Used for generic status messages.
// ///ERROR: Used to indicate something has gone wrong.
// */
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Level {
    TRACE,
    DEBUG,
    WARN,
    INFO,
    ERROR,
}

///Display implmentation for `Level` - allowing us to
///use {} instead of {:?} in
impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Level::TRACE => write!(f, "Trace"),
            Level::DEBUG => write!(f, "Debug"),
            Level::WARN => write!(f, "Warn"),
            Level::INFO => write!(f, "Info"),
            Level::ERROR => write!(f, "Error"),
        }
    }
}

///The `Logger` trait is an abstraction for concrete loggers e.g. (tokio trace)
pub trait Logger {
    fn new() -> Self;
    fn initialize(&mut self, level: Level);
    fn log(&self, level: Level, message: String);
    fn get_log_level(&self) -> Level;
}

///Exports of the concrete loggers
///SimpleLogger is a *very* basic logger, mostly used for testing the interface
///output is to console.
pub mod simple;

///TraceLogger is Tokio Trace's logger with a Subscriber implementation for Essembly
pub mod trace;

//Experimental - i18n
#[derive(Debug)]
pub enum Locale {
    TH_TH,
    EN_US,
    EN_GB,
}

//This is just a test
//This should be done with a real localization process.  See locale in this crate
pub fn translate(locale: Locale, level: Level) -> &'static str {
    match locale {
        Locale::TH_TH => TH_TH_HASHMAP.get(&level).unwrap(),
        Locale::EN_US => EN_US_HASHMAP.get(&level).unwrap(),
        _ => EN_US_HASHMAP.get(&level).unwrap(),
    }
}

lazy_static! {
    static ref TH_TH_HASHMAP: HashMap<Level, &'static str> = {
        let mut m = HashMap::new();
        m.insert(Level::TRACE, "ติดตาม");
        m.insert(Level::DEBUG, "ตรวจแก้จุดบกพร่องที่ผิด");
        m.insert(Level::INFO, "ข้อมูล");
        m.insert(Level::WARN, "แจ้งเตือน");
        m.insert(Level::ERROR, "ความผิดพลาด");
        m
    };
}

lazy_static! {
    static ref EN_US_HASHMAP: HashMap<Level, &'static str> = {
        let mut m = HashMap::new();
        m.insert(Level::TRACE, "trace");
        m.insert(Level::DEBUG, "debug");
        m.insert(Level::INFO, "info");
        m.insert(Level::ERROR, "error");
        m
    };
}
