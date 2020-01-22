use crate::types::{
    il2cpp_array_size_t, EventInfo, FieldInfo, Il2CppArray, Il2CppAssembly, Il2CppChar,
    Il2CppClass, Il2CppImage, Il2CppMemoryCallbacks, Il2CppMethodPointer, Il2CppReflectionType,
    Il2CppType, MethodInfo, PropertyInfo,
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
    pub bounded_array_class_get:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, u32, bool) -> *mut Il2CppClass>,
    pub array_element_size: Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass) -> c_int>,
    pub assembly_get_image:
        Symbol<'l, unsafe extern "C" fn(*const Il2CppAssembly) -> *const Il2CppImage>,
    pub class_enum_basetype:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass) -> *const Il2CppType>,
    pub class_is_generic: Symbol<'l, unsafe extern "C" fn(*const Il2CppClass) -> bool>,
    pub class_is_inflated: Symbol<'l, unsafe extern "C" fn(*const Il2CppClass) -> bool>,
    pub class_is_assignable_from:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, *mut Il2CppClass) -> bool>,
    pub class_is_subclass_of:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, *mut Il2CppClass, bool) -> bool>,
    pub class_has_parent:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, *mut Il2CppClass) -> bool>,
    pub class_from_il2cpp_type:
        Symbol<'l, unsafe extern "C" fn(*const Il2CppType) -> *mut Il2CppClass>,
    pub class_from_name: Symbol<
        'l,
        unsafe extern "C" fn(*const Il2CppImage, *const c_char, *const c_char) -> *mut Il2CppClass,
    >,
    pub class_from_system_type:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppReflectionType) -> *mut Il2CppClass>,
    pub class_get_element_lass:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass) -> *mut Il2CppClass>,
    pub class_get_events:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, *mut c_void) -> *const EventInfo>,
    pub class_get_fields:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, *mut c_void) -> *mut FieldInfo>,
    pub class_get_nested_types:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, *mut c_void) -> *mut Il2CppClass>,
    pub class_get_interfaces:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, *mut c_void) -> *mut Il2CppClass>,
    pub class_get_properties:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, *mut c_void) -> *const PropertyInfo>,
    pub class_get_property_from_name:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, *const c_char) -> *const PropertyInfo>,
    pub class_get_field_from_name:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, *const c_char) -> *mut FieldInfo>,
    pub class_get_methods:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, *mut c_void) -> *const MethodInfo>,
    pub class_get_method_from_name: Symbol<
        'l,
        unsafe extern "C" fn(*mut Il2CppClass, *const c_char, c_int) -> *const MethodInfo,
    >,
    pub class_get_name: Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass) -> *const c_char>,
    pub class_get_namespace: Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass) -> *const c_char>,
    pub class_get_parent: Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass) -> *mut Il2CppClass>,
    pub class_get_declaring_type:
        Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass) -> *mut Il2CppClass>,
    pub class_instance_size: Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass) -> i32>,
    pub class_num_fields: Symbol<'l, unsafe extern "C" fn(*const Il2CppClass) -> usize>,
    pub class_is_valuetype: Symbol<'l, unsafe extern "C" fn(*const Il2CppClass) -> bool>,
    pub class_value_size: Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass, u32) -> i32>,
    pub class_is_blittable: Symbol<'l, unsafe extern "C" fn(*const Il2CppClass) -> bool>,
    pub class_get_flags: Symbol<'l, unsafe extern "C" fn(*const Il2CppClass) -> c_int>,
    pub class_is_abstract: Symbol<'l, unsafe extern "C" fn(*const Il2CppClass) -> bool>,
    pub class_is_interface: Symbol<'l, unsafe extern "C" fn(*const Il2CppClass) -> bool>,
    pub class_array_element_size:
        Symbol<'l, unsafe extern "C" fn(*const Il2CppClass, u32) -> c_int>,
    pub class_from_type: Symbol<'l, unsafe extern "C" fn(*const Il2CppType) -> *mut Il2CppClass>,
    pub class_get_type: Symbol<'l, unsafe extern "C" fn(*mut Il2CppClass) -> *const Il2CppType>,
}
