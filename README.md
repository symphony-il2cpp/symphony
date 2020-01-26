# bshook

## Dependencies

* Rust
* NDK

## Setup

```
rustup target install aarch64-linux-android
<NDK>/build/tools/make_standalone_toolchain.py --api 25 --arch arm64 --install-dir resources/ndk
```

## C FFI bindgens

### Dependencies

* LLVM
* bindgen

### Setup

```
bindgen il2cpp/resources/wrapper.h -o il2cpp/resources/bindgen.rs --generate types,vars -- -Iresources/libil2cpp
bindgen inline_hook/resources/wrapper.h -o inline_hook/resources/bindgen.rs --whitelist-function A64HookFunctionV?
```
