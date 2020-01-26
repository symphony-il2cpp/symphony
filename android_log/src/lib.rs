#![allow(non_snake_case)]

use std::os::raw::c_int;

/// Verbose logging. Should typically be disabled for a release apk.
pub const ANDROID_LOG_VERBOSE: c_int = 2;
/// Debug logging. Should typically be disabled for a release apk.
pub const ANDROID_LOG_DEBUG: c_int = 3;
/// Informational logging. Should typically be disabled for a release apk.
pub const ANDROID_LOG_INFO: c_int = 4;
/// Warning logging. For use with recoverable failures.
pub const ANDROID_LOG_WARN: c_int = 5;
/// Error logging. For use with unrecoverable failures.
pub const ANDROID_LOG_ERROR: c_int = 6;
/// Fatal logging. For use when aborting.
pub const ANDROID_LOG_FATAL: c_int = 7;

include!(concat!("../resources/bindgen.rs"));
