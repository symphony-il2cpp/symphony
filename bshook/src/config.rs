pub use bshook_derive::Config;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;

pub const CONFIG_PATH: &str = "/sdcard/Android/data/com.beatgames.beatsaber/files/mod_cfgs/";

pub trait Config: Serialize + DeserializeOwned {
    fn filepath() -> &'static str;
    fn load() -> Result<Self, error::Error> {
        let f = File::open(Self::filepath())?;
        Ok(serde_json::from_reader(f)?)
    }
    fn reload(&mut self) -> Result<(), error::Error> {
        let f = File::open(Self::filepath())?;
        *self = serde_json::from_reader(f)?;
        Ok(())
    }
    fn write(&self) -> Result<(), error::Error> {
        let f = File::create(Self::filepath())?;
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
