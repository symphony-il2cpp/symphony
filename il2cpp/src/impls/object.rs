use crate::{
    error::Error,
    functions::IL2CPP_SO,
    types::{Il2CppClass, Il2CppObject},
};

impl Il2CppObject {
    pub fn new<'a>(class: &Il2CppClass) -> Result<&'a mut Self, Error> {
        let instance = unsafe { IL2CPP_SO.il2cpp_object_new(class) };
        if instance.is_null() {
            return Err(Error::NullPointer(
                "couldn't create new string instance".to_owned(),
            ));
        }
        Ok(unsafe { &mut *instance })
    }
}
