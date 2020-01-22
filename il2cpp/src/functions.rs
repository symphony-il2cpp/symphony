use crate::types::{
    il2cpp_array_size_t, Il2CppArray, Il2CppChar, Il2CppClass, Il2CppImage, Il2CppMemoryCallbacks,
    Il2CppMethodPointer,
};
use libloading::{Library, Symbol};
use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

pub const IL2CPP_SO_PATH: &str = "/data/app/com.beatgames.beatsaber-1/lib/arm64/libil2cpp.so";

pub struct Il2CppFunctions<'l> {
    library: Library,
    pub init: Symbol<'l, unsafe extern "C" fn(*const c_char)>,
    pub init_utf16: Symbol<'l, unsafe extern "C" fn(*const Il2CppChar)>,
    pub shutdown: Symbol<'l, unsafe extern "C" fn()>,
    pub set_config_dir: Symbol<'l, unsafe extern "C" fn(*const c_char)>,
    pub set_data_dir: Symbol<'l, unsafe extern "C" fn(*const c_char)>,
    pub set_temp_dir: Symbol<'l, unsafe extern "C" fn(*const c_char)>,
    pub set_commandline_arguments:
        Symbol<'l, unsafe extern "C" fn(c_int, *const *const c_char, *const c_char)>,
    pub set_commandline_arguments_utf16:
        Symbol<'l, unsafe extern "C" fn(c_int, *const *const Il2CppChar, *const c_char)>,
    pub set_config: Symbol<'l, unsafe extern "C" fn(*const c_char)>,
    pub set_config_utf16: Symbol<'l, unsafe extern "C" fn(*const Il2CppChar)>,
    pub set_memory_callbacks: Symbol<'l, unsafe extern "C" fn(*mut Il2CppMemoryCallbacks)>,
    pub get_corlib: Symbol<'l, unsafe extern "C" fn() -> *const Il2CppImage>,
    pub add_internal_call: Symbol<'l, unsafe extern "C" fn(*const c_char, Il2CppMethodPointer)>,
    pub resolve_icall: Symbol<'l, unsafe extern "C" fn(*const c_char) -> Il2CppMethodPointer>,
    pub alloc: Symbol<'l, unsafe extern "C" fn(usize) -> *mut c_void>,
    pub free: Symbol<'l, unsafe extern "C" fn(*mut c_void)>,
    pub array_class_get:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, u32) -> *mut Il2CppClass>,
    pub array_length: Symbol<'l, unsafe extern "C" fn(*mut Il2CppArray) -> u32>,
    pub array_get_byte_length: Symbol<'l, unsafe extern "C" fn(*mut Il2CppArray) -> u32>,
    pub array_new:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, il2cpp_array_size_t) -> *mut Il2CppArray>,
    pub array_new_specific:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, il2cpp_array_size_t) -> *mut Il2CppArray>,
    pub array_new_full: Symbol<
        'l,
        unsafe extern "C" fn(
            *mut Il2CppClass,
            il2cpp_array_size_t,
            il2cpp_array_size_t,
        ) -> *mut Il2CppArray,
    >,
}
