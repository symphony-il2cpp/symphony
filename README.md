# rust-bshook

## Dependencies

* LLVM
* Rust
* bindgen
* NDK

## Setup

```
rustup target install aarch64-linux-android
<NDK>/build/tools/make_standalone_toolchain.py --api 25 --arch arm64 --install-dir tools/ndk
```
