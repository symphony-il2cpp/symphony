use bshook::{
    error,
    il2cpp::{types::Il2CppObject, utils},
    info, inline_hook,
};
use std::ffi::c_void;

extern "C" fn hook(this: *mut Il2CppObject, first_activation: bool, activation_type: i32) {
    info!("We're hooked in!");
    unsafe {
        ORIGINAL(this, first_activation, activation_type);
    }
}
static mut ORIGINAL: extern "C" fn(
    this: *mut Il2CppObject,
    first_activation: bool,
    activation_type: i32,
) = hook;

#[no_mangle]
pub extern "C" fn load() {
    info!("Installing offsetless hook");
    let method = match unsafe { utils::find_method("", "FireworksController", "OnEnable", 0) } {
        Ok(m) => m,
        Err(e) => {
            error!("{:#?}", e);
            return;
        }
    };
    unsafe {
        inline_hook::A64HookFunction(
            (*method).methodPointer.unwrap() as *mut c_void,
            hook as *mut c_void,
            &mut ORIGINAL as *mut _ as *mut *mut c_void,
        )
    }
}
