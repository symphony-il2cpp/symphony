use crate::types::Il2CppException;
use std::ops::{Deref, DerefMut};

pub struct Exception(*mut Il2CppException);

impl<T: std::error::Error> From<T> for Exception {
    fn from(error: T) -> Self {
        let exception: *mut Il2CppException = Il2CppException::new(error.description()).unwrap();
        Self(exception)
    }
}
impl From<&mut Il2CppException> for Exception {
    fn from(ex: &mut Il2CppException) -> Self {
        Self(ex)
    }
}

impl Deref for Exception {
    type Target = Il2CppException;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
impl DerefMut for Exception {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}
