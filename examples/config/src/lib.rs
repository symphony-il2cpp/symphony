use bshook::{config::Config, logging};
use log::{error, info};
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

    let config = ModConfig::load();
    match config {
        Ok(c) => info!("Config loaded: {:#?}", c),
        Err(_) => {
            info!("Error loading config, creating a new one");
            let c = ModConfig {
                name: "Raphaël Thériault".to_owned(),
                age: 99,
            };
            match c.write() {
                Ok(_) => info!("Config created: {:#?}", c),
                Err(e) => error!("Error creating config: {}", e),
            }
        }
    }
}
