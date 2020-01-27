use crate::types::{Il2CppAssembly, Il2CppClass, Il2CppDomain, Il2CppImage, MethodInfo};
use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use lazy_static::lazy_static;
use std::{
    ffi::CString,
    os::raw::{c_char, c_int},
    process,
};
use symphony_android_log::{self, ANDROID_LOG_FATAL};

pub const IL2CPP_SO_PATH: &str = "/data/app/com.beatgames.beatsaber-1/lib/arm64/libil2cpp.so";

#[derive(WrapperApi)]
pub struct Il2CppSO {
    il2cpp_assembly_get_image:
        unsafe extern "C" fn(assembly: *const Il2CppAssembly) -> *const Il2CppImage,
    il2cpp_class_from_name: unsafe extern "C" fn(
        image: *const Il2CppImage,
        name: *const c_char,
        namespace: *const c_char,
    ) -> *mut Il2CppClass,
    il2cpp_class_get_method_from_name: unsafe extern "C" fn(
        class: *mut Il2CppClass,
        name: *const c_char,
        args_count: c_int,
    ) -> *const MethodInfo,
    il2cpp_class_get_name: unsafe extern "C" fn(class: *mut Il2CppClass) -> *const c_char,
    il2cpp_class_get_namespace: unsafe extern "C" fn(class: *mut Il2CppClass) -> *const c_char,
    il2cpp_domain_get: unsafe extern "C" fn() -> *mut Il2CppDomain,
    il2cpp_domain_get_assemblies: unsafe extern "C" fn(
        domain: *const Il2CppDomain,
        size: *mut usize,
    ) -> *const *mut Il2CppAssembly,
}

lazy_static! {
    pub static ref IL2CPP_SO: Container<Il2CppSO> = unsafe {
        Container::load(IL2CPP_SO_PATH).unwrap_or_else(|e| {
            let tag = CString::new("symphony").unwrap();
            let message = CString::new(format!("{:#}", e)).unwrap();
            symphony_android_log::__android_log_write(
                ANDROID_LOG_FATAL,
                tag.as_ptr(),
                message.as_ptr(),
            );
            process::exit(1);
        })
    };
}
