use crate::{error::Error, types::Il2CppArray};
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub struct GenericIl2CppArray<T>(*mut Il2CppArray, PhantomData<T>);

impl<T> GenericIl2CppArray<T> {
    pub fn as_slice(&self) -> Result<&[T], Error> {
        unsafe { (*self.0).as_slice() }
    }

    pub fn as_mut_slice(&mut self) -> Result<&mut [T], Error> {
        unsafe { (*self.0).as_mut_slice() }
    }
}

impl<T> Deref for GenericIl2CppArray<T> {
    type Target = Il2CppArray;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
impl<T> DerefMut for GenericIl2CppArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl<T> From<*mut Il2CppArray> for GenericIl2CppArray<T> {
    fn from(a: *mut Il2CppArray) -> Self {
        Self(a, PhantomData)
    }
}
