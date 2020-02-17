use crate::{
    error::Error,
    types::{kIl2CppSizeOfArray, Il2CppArray},
};
use std::slice;

impl Il2CppArray {
    pub unsafe fn as_slice<T>(&self) -> Result<&[T], Error> {
        if self.bounds.is_null() {
            return Err(Error::NullPointer(
                "can't convert Il2CppArray with missing bounds to slice".to_owned(),
            ));
        }

        let ptr = ((self as *const Self as isize) + (kIl2CppSizeOfArray as isize)) as *const T;
        let len = (*self.bounds).length;
        Ok(slice::from_raw_parts(ptr, len))
    }

    pub unsafe fn as_mut_slice<T>(&mut self) -> Result<&mut [T], Error> {
        if self.bounds.is_null() {
            return Err(Error::NullPointer(
                "can't convert Il2CppArray with missing bounds to slice".to_owned(),
            ));
        }

        let ptr = ((self as *mut Self as isize) + (kIl2CppSizeOfArray as isize)) as *mut T;
        let len = (*self.bounds).length;
        Ok(slice::from_raw_parts_mut(ptr, len))
    }
}
