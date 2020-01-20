use std::{env, path::Path, process::Command};

fn main() {
    println!("cargo:rustc-link-lib=static=il2cpp");
    println!(
        "cargo:rustc-link-search=native={}",
        concat!(env!("CARGO_MANIFEST_DIR"), "/obj/local/arm64-v8a")
    );

    let out_dir = env::var("OUT_DIR").unwrap();
    let bindgen_path = Path::new(&out_dir)
        .join("bindgen.rs")
        .to_str()
        .unwrap()
        .to_owned();

    if !Command::new("bindgen")
        .args(&["wrapper.h", "-o", &bindgen_path, "--", "-Ivendor/libil2cpp"])
        .status()
        .unwrap()
        .success()
    {
        panic!("Failed to generate bindings");
    }

    #[cfg(windows)]
    {
        if !Command::new("cmd")
            .args(&[
                "/C",
                "ndk-build.cmd NDK_PROJECT_PATH=. APP_BUILD_SCRIPT=./Android.mk NDK_APPLICATION_MK=./Application.mk",
            ])
            .status()
            .unwrap()
            .success()
        {
            panic!("Failed to build il2cpp");
        }
    }
    #[cfg(not(windows))]
    {
        if !Command::new("sh")
            .args(&[
                "-c",
                "ndk-build NDK_PROJECT_PATH=. APP_BUILD_SCRIPT=./Android.mk NDK_APPLICATION_MK=./Application.mk",
            ])
            .status()
            .unwrap()
            .success()
        {
            panic!("Failed to build il2cpp");
        }
    }
}
