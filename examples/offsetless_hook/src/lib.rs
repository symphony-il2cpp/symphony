use bshook::{
    error,
    il2cpp::{types::Il2CppObject, utils},
    info, inline_hook,
};
use std::ffi::c_void;

extern "C" fn hook(this: *mut Il2CppObject, first_activation: bool, activation_type: i32) {
    info!("We're hooked in!");
    original(this, first_activation, activation_type);
}
#[allow(unused_variables)]
extern "C" fn original(this: *mut Il2CppObject, first_activation: bool, activation_type: i32) {}

#[no_mangle]
pub extern "C" fn load() {
    info!("Installing offsetless hook");
    let method =
        match unsafe { utils::find_method("", "HealthWarningFlowCoordinator", "DidActivate", 2) } {
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
            &mut original as *mut _ as *mut *mut c_void,
        )
    }
}
