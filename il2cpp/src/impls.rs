use crate::{
    error::Error,
    types::{kIl2CppSizeOfArray, Il2CppArray, Il2CppString},
};
use std::slice;

impl Il2CppString {
    pub fn to_string(&self) -> Result<String, Error> {
        let slice = unsafe { self.chars.as_slice(self.length as usize) };
        Ok(String::from_utf16(slice)?)
    }
}

impl Il2CppArray {
    pub fn as_slice<T>(&self) -> Result<&[T], Error> {
        if self.bounds.is_null() {
            return Err(Error::NullPointer(
                "can't convert Il2CppArray with missing bounds to slice".to_owned(),
            ));
        }

        let ptr = ((self as *const Self as isize) + (kIl2CppSizeOfArray as isize)) as *const T;
        let len = unsafe { *self.bounds }.length;
        Ok(unsafe { slice::from_raw_parts(ptr, len) })
    }

    pub fn as_slice_mut<T>(&mut self) -> Result<&mut [T], Error> {
        if self.bounds.is_null() {
            return Err(Error::NullPointer(
                "can't convert Il2CppArray with missing bounds to slice".to_owned(),
            ));
        }

        let ptr = ((self as *mut Self as isize) + (kIl2CppSizeOfArray as isize)) as *mut T;
        let len = unsafe { *self.bounds }.length;
        Ok(unsafe { slice::from_raw_parts_mut(ptr, len) })
    }
}
