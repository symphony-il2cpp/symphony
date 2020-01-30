use crate::{error::Error, types::Il2CppString};

impl Il2CppString {
    pub fn to_string(&self) -> Result<String, Error> {
        let slice = unsafe { self.chars.as_slice(self.length as usize) };
        Ok(String::from_utf16(slice)?)
    }
}
