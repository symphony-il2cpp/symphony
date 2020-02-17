use crate::{
    args,
    error::Error,
    types::{Il2CppClass, Il2CppObject, Il2CppString},
};

impl Il2CppString {
    pub fn new<'a>(contents: &str) -> Result<&'a mut Self, Error> {
        Self::from_utf16(&contents.encode_utf16().collect::<Vec<u16>>())
    }
    pub fn from_utf16<'a>(utf16: &[u16]) -> Result<&'a mut Self, Error> {
        let class = Il2CppClass::find("System", "String")?;
        let instance = Il2CppObject::new(class)?;

        let ctor = class.get_method(".ctor", 3)?;
        ctor.invoke(Some(instance), args!(utf16.as_ptr()))?;

        Ok(unsafe { &mut *(instance as *mut _ as *mut Self) })
    }

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
