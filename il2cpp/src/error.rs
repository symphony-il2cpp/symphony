use crate::types::Il2CppException;
use std::{ffi::NulError, str::Utf8Error, string::FromUtf16Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Null pointer ({0})")]
    NullPointer(String),
    #[error("Nul byte in C string")]
    NulByte(#[from] NulError),
    #[error("UTF8 error")]
    Utf8(#[from] Utf8Error),
    #[error("UTF16 error")]
    FromUtf16(#[from] FromUtf16Error),
    #[error("Il2Cpp exception")]
    Il2CppException(#[from] Il2CppException),
}
