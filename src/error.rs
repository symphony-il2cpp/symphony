use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IO(#[from] io::Error),
    #[error("Invalid JSON")]
    JSON(#[from] serde_json::Error),
    #[error("il2cpp error")]
    Il2Cpp(#[from] symphony_il2cpp::error::Error),
}
