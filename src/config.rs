use crate::error::Error;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;
pub use symphony_derive::Config;

pub const CONFIG_PATH: &str = "/sdcard/Android/data/com.beatgames.beatsaber/files/mod_cfgs/";

pub trait Config: Serialize + DeserializeOwned {
    fn filepath() -> &'static str;
    fn load() -> Result<Self, Error> {
        let f = File::open(Self::filepath())?;
        Ok(serde_json::from_reader(f)?)
    }
    fn reload(&mut self) -> Result<(), Error> {
        let f = File::open(Self::filepath())?;
        *self = serde_json::from_reader(f)?;
        Ok(())
    }
    fn write(&self) -> Result<(), Error> {
        let f = File::create(Self::filepath())?;
        serde_json::to_writer_pretty(f, self)?;
        Ok(())
    }
}
