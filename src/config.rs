use result::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;

const CONFIG_PATH: &str = "/sdcard/Android/data/com.beatgames.beatsaber/files/mod_cfgs/";

pub trait Configuration: Serialize + DeserializeOwned {
    fn filename() -> &'static str;
    fn load() -> Result<Self> {
        let f = File::open(format!("{}{}.json", CONFIG_PATH, Self::filename()))?;
        Ok(serde_json::from_reader(f)?)
    }
    fn reload(&mut self) -> Result<()> {
        let f = File::open(format!("{}{}.json", CONFIG_PATH, Self::filename()))?;
        *self = serde_json::from_reader(f)?;
        Ok(())
    }
    fn write(&self) -> Result<()> {
        let f = File::create(format!("{}{}.json", CONFIG_PATH, Self::filename()))?;
        serde_json::to_writer_pretty(f, self)?;
        Ok(())
    }
}

pub mod error {
    use std::io;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("IO error")]
        IO(#[from] io::Error),
        #[error("Invalid JSON")]
        JSON(#[from] serde_json::Error),
    }
}

pub mod result {
    use crate::config::error::Error;
    pub type Result<T> = std::result::Result<T, Error>;
}
