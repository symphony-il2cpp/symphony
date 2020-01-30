use std::{ffi::NulError, str::Utf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Null pointer ({0})")]
    NullPointer(String),
    #[error("Nul byte in C string")]
    NulByte(#[from] NulError),
    #[error("UTF8 error")]
    Utf8(#[from] Utf8Error),
}
