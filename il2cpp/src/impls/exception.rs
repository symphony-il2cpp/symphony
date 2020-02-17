use crate::{
    args,
    error::Error,
    functions::IL2CPP_SO,
    types::{Il2CppClass, Il2CppException, Il2CppObject, Il2CppString},
};
use std::{
    ffi::{CStr, CString},
    fmt::{self, Debug, Display, Formatter},
};

const EXCEPTION_MESSAGE_SIZE: usize = 4096;

impl Il2CppException {
    pub fn new<'a>(message: &str) -> Result<&'a mut Self, Error> {
        let class = Il2CppClass::find("System", "Exception")?;
        let instance = Il2CppObject::new(class)?;

        let message: *mut Il2CppString = Il2CppString::new(message)?;

        let ctor = class.get_method(".ctor", 1)?;
        ctor.invoke(Some(instance), args!(message))?;

        Ok(unsafe { &mut *(instance as *mut _ as *mut Self) })
    }
}

impl Display for Il2CppException {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let message = &mut [0; EXCEPTION_MESSAGE_SIZE];
        unsafe {
            IL2CPP_SO.il2cpp_format_exception(
                self,
                message.as_mut_ptr(),
                EXCEPTION_MESSAGE_SIZE as i32,
            );
        }
        let unknown_c_string = CString::new("Unknown il2cpp exception").unwrap();
        let message = CStr::from_bytes_with_nul(unsafe { &*(message as *mut [i8] as *mut [u8]) })
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
