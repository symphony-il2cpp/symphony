fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if &target_os == "android" {
        cc::Build::new()
            .file("resources/wrapper.h")
            .compile("inlinehook");
    }
}
