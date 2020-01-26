# bshook

## Requirements

* Rust
* NDK

## Setup

```
rustup target install aarch64-linux-android
<NDK>/build/tools/make_standalone_toolchain.py --api 25 --arch arm64 --install-dir resources/ndk
```

## C FFI bindgens

### Requirements

* LLVM
* bindgen

### Generation

```
bindgen android_log/resources/wrapper.h -o android_log/resources/bindgen.rs --whitelist-function __android_log_write -- -I"<NDK>/sysroot/usr/include"
bindgen il2cpp/resources/wrapper.h -o il2cpp/resources/bindgen.rs --generate types,vars -- -I"resources/libil2cpp"
bindgen inline_hook/resources/wrapper.h -o inline_hook/resources/bindgen.rs --whitelist-function A64HookFunctionV?
```
