use crate::{
    error::Error,
    functions::IL2CPP_SO,
    types::{Il2CppClass, MethodInfo},
};
use std::{
    ffi::{CStr, CString},
    slice,
};

pub fn get_class_from_name<'a>(namespace: &str, name: &str) -> Result<&'a mut Il2CppClass, Error> {
    let domain = unsafe { IL2CPP_SO.il2cpp_domain_get() };
    if domain.is_null() {
        return Err(Error::NullReturn("could not get domain".to_owned()));
    }

    let mut assembly_count = 0;
    let all_assemblies =
        unsafe { IL2CPP_SO.il2cpp_domain_get_assemblies(domain, &mut assembly_count) };
    let all_assemblies = unsafe { slice::from_raw_parts(all_assemblies, assembly_count) };

    for assembly in all_assemblies {
        if assembly.is_null() {
            return Err(Error::NullReturn("could not get all assemblies".to_owned()));
        }

        let image = unsafe { IL2CPP_SO.il2cpp_assembly_get_image(*assembly) };
        if image.is_null() {
            return Err(Error::NullReturn(format!(
                "assembly with name {:?} has a null image",
                unsafe { CStr::from_ptr((*(*assembly)).aname.name) }.to_str()?
            )));
        }

        let class = unsafe {
            IL2CPP_SO.il2cpp_class_from_name(
                image,
                CString::new(namespace)?.as_ptr(),
                CString::new(name)?.as_ptr(),
            )
        };
        if !class.is_null() {
            return Ok(unsafe { &mut *class });
        }
    }
    Err(Error::NullReturn(format!(
        "couldn't find class {:?} (namespace {:?})",
        name, namespace,
    )))
}

pub fn find_method<'a>(
    namespace: &str,
    class_name: &str,
    method_name: &str,
    argument_count: i32,
) -> Result<&'a MethodInfo, Error> {
    let class = get_class_from_name(namespace, class_name)?;

    let method_info = unsafe {
        IL2CPP_SO.il2cpp_class_get_method_from_name(
            class,
            CString::new(method_name)?.as_ptr(),
            argument_count,
        )
    };
    if method_info.is_null() {
        return Err(Error::NullReturn(format!(
            "couldn't find method {:?} with {} parameters in class {:?} (namespace {:?})",
            method_name, argument_count, class_name, namespace,
        )));
    }
    Ok(unsafe { &*method_info })
}
