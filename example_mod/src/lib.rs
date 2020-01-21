use bshook::{config::Config, logging};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Config, Serialize, Deserialize, Debug)]
#[config(filename = "exampleModConfig.json")]
struct ModConfig {
    name: String,
    age: u8,
}

#[no_mangle]
pub extern "C" fn load() {
    logging::init("Example Mod");

    let config = ModConfig::load().unwrap();
    info!("{:#?}", config);
}
