use result::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;

const CONFIG_PATH: &str = "/sdcard/Android/data/com.beatgames.beatsaber/files/mod_cfgs/";

pub trait Config: Serialize + DeserializeOwned {
    fn filename() -> &'static str;
    fn filepath() -> String {
        format!("{}{}.json", CONFIG_PATH, Self::filename())
    }
    fn load() -> Result<Self> {
        let f = File::open(Self::filepath())?;
        Ok(serde_json::from_reader(f)?)
    }
    fn reload(&mut self) -> Result<()> {
        let f = File::open(Self::filepath())?;
        *self = serde_json::from_reader(f)?;
        Ok(())
    }
    fn write(&self) -> Result<()> {
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

pub mod result {
    use crate::config::error::Error;
    pub type Result<T> = std::result::Result<T, Error>;
}
