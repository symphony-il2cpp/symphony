use std::ffi::c_void;
use symphony::{
    debug, error,
    il2cpp::{types::Il2CppObject, utils},
    info, inline_hook,
};

extern "C" fn hook(this: *mut Il2CppObject, first_activation: bool, activation_type: i32) {
    info!("We're hooked in!");
    unsafe {
        original(this, first_activation, activation_type);
    }
}
extern "C" {
    fn original(this: *mut Il2CppObject, first_activation: bool, activation_type: i32);
}

#[no_mangle]
pub extern "C" fn init() {
    debug!("Init");
}
#[no_mangle]
pub extern "C" fn preload() {
    debug!("Preload");
}

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
    debug!("Method info pointer: {:#?}", method);

    let method_pointer = match unsafe { (*method).methodPointer } {
        Some(p) => p,
        None => {
            error!("No method pointer");
            return;
        }
    };

    debug!("Method pointer: {:#?}", method_pointer);
    unsafe {
        inline_hook::A64HookFunction(
            method_pointer as *mut c_void,
            hook as *mut c_void,
            &mut (original as *mut c_void),
        )
    }
}
