use std::{ffi::NulError, str::Utf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Null return value ({0})")]
    NullReturn(String),
    #[error("Null function parameter ({0})")]
    NullParameter(String),
    #[error("Nul byte in C string")]
    NulByte(#[from] NulError),
    #[error("UTF8 error")]
    Utf8(#[from] Utf8Error),
}
