use std::ffi::OsString;

// General variables

pub static LS_COLORS: &str = "LS_COLORS";

pub static COLUMNS: &str = "COLUMNS";

pub static TIME_STYLE: &str = "TIME_STYLE";

pub static PERF_COLORS: &str = "PERF_COLORS";

pub static PERF_STRICT: &str = "PERF_STRICT";

pub static PERF_DEBUG: &str = "PERF_DEBUG";

pub static PERF_GRID_ROWS: &str = "PERF_GRID_ROWS";

pub trait Vars {
    fn get(&self, name: &'static str) -> Option<OsString>;
}

#[cfg(test)]
impl Vars for Option<OsString> {
    fn get(&self, _name: &'static str) -> Option<OsString> {
        self.clone()
    }
}
