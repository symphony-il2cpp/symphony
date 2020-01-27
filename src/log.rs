use std::{ffi::CString, os::raw::c_int};
use symphony_android_log::{
    self, ANDROID_LOG_DEBUG, ANDROID_LOG_ERROR, ANDROID_LOG_FATAL, ANDROID_LOG_INFO,
    ANDROID_LOG_VERBOSE, ANDROID_LOG_WARN,
};

pub enum LogLevel {
    Verbose,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}
impl Into<c_int> for LogLevel {
    fn into(self) -> c_int {
        match self {
            LogLevel::Verbose => ANDROID_LOG_VERBOSE,
            LogLevel::Debug => ANDROID_LOG_DEBUG,
            LogLevel::Info => ANDROID_LOG_INFO,
            LogLevel::Warn => ANDROID_LOG_WARN,
            LogLevel::Error => ANDROID_LOG_ERROR,
            LogLevel::Fatal => ANDROID_LOG_FATAL,
        }
    }
}

pub fn log(level: LogLevel, tag: &str, message: &str) {
    let prio = level.into();
    let tag = CString::new(tag).unwrap();
    let text = CString::new(message).unwrap();
    unsafe {
        symphony_android_log::__android_log_write(prio, tag.as_ptr(), text.as_ptr());
    }
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => {
        $crate::log::log(
            $crate::log::LogLevel::Verbose,
            concat!(
                "symphony [",
                env!("CARGO_PKG_NAME"),
                " v",
                env!("CARGO_PKG_VERSION"),
                "]",
            ),
            &format!($($arg)*),
        )
    };
}
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::log(
            $crate::log::LogLevel::Debug,
            concat!(
                "symphony [",
                env!("CARGO_PKG_NAME"),
                " v",
                env!("CARGO_PKG_VERSION"),
                "]",
            ),
            &format!($($arg)*),
        )
    };
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::log(
            $crate::log::LogLevel::Info,
            concat!(
                "symphony [",
                env!("CARGO_PKG_NAME"),
                " v",
                env!("CARGO_PKG_VERSION"),
                "]",
            ),
            &format!($($arg)*),
        )
    };
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log::log(
            $crate::log::LogLevel::Warn,
            concat!(
                "symphony [",
                env!("CARGO_PKG_NAME"),
                " v",
                env!("CARGO_PKG_VERSION"),
                "]",
            ),
            &format!($($arg)*),
        )
    };
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::log(
            $crate::log::LogLevel::Error,
            concat!(
                "symphony [",
                env!("CARGO_PKG_NAME"),
                " v",
                env!("CARGO_PKG_VERSION"),
                "]",
            ),
            &format!($($arg)*),
        )
    };
}
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::log::log(
            $crate::log::LogLevel::Fatal,
            concat!(
                "symphony [",
                env!("CARGO_PKG_NAME"),
                " v",
                env!("CARGO_PKG_VERSION"),
                "]",
            ),
            &format!($($arg)*),
        )
    };
}
