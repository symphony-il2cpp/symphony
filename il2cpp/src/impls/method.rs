use crate::{
    error::Error,
    functions::IL2CPP_SO,
    types::{Il2CppException, Il2CppObject, MethodInfo},
};
use std::{os::raw::c_void, ptr};

impl MethodInfo {
    pub fn invoke<'a>(
        &self,
        instance: Option<&mut Il2CppObject>,
        args: &mut [*mut c_void],
    ) -> Result<Option<&'a mut Il2CppObject>, Error> {
        let mut exception: *mut Il2CppException = ptr::null_mut();
        let instance = match instance {
            Some(i) => i as *mut _ as *mut c_void,
            None => ptr::null_mut(),
        };
        let ret = unsafe {
            IL2CPP_SO.il2cpp_runtime_invoke(self, instance, args.as_mut_ptr(), &mut exception)
        };
        if exception.is_null() {
            if !ret.is_null() {
                Ok(Some(unsafe { &mut *ret }))
            } else {
                Ok(None)
            }
        } else {
            Err(unsafe { *exception }.into())
        }
    }
}
