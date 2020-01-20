use android_logger::Config;
pub use log::{debug, error, info, trace, warn};

pub fn init(tag: &str) {
    android_logger::init_once(Config::default().with_tag(tag));
}
