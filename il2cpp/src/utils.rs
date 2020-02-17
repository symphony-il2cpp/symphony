#[macro_export]
macro_rules! args {
    ($($arg:expr),*) => {
        &mut [$($arg as *mut std::os::raw::c_void),*]
    };
}
