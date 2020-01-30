use crate::{
    error::Error,
    functions::IL2CPP_SO,
    types::{kIl2CppSizeOfArray, Il2CppArray, Il2CppException, Il2CppString},
};
use std::{
    ffi::{CStr, CString},
    fmt::{self, Debug, Display, Formatter},
    slice,
};

impl Il2CppString {
    pub fn as_utf16(&self) -> &[u16] {
        unsafe { self.chars.as_slice(self.length as usize) }
    }

    pub fn as_utf16_mut(&mut self) -> &mut [u16] {
        unsafe { self.chars.as_mut_slice(self.length as usize) }
    }

    pub fn to_string(&self) -> Result<String, Error> {
        Ok(String::from_utf16(self.as_utf16())?)
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

const EXCEPTION_MESSAGE_SIZE: usize = 4096;

impl Display for Il2CppException {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let mut message = [0; EXCEPTION_MESSAGE_SIZE];
        unsafe {
            IL2CPP_SO.il2cpp_format_exception(
                self,
                message.as_mut_ptr(),
                EXCEPTION_MESSAGE_SIZE as i32,
            );
        }
        let unknown_c_string = CString::new("Unknown il2cpp exception").unwrap();
        let message = CStr::from_bytes_with_nul(&message)
            .unwrap_or(&unknown_c_string)
            .to_str()
            .unwrap_or("Unknown il2cpp exception");
        write!(f, "{}", message)
    }
}

impl Debug for Il2CppException {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Il2CppException {}
