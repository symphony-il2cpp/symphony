fn main() {
    cc::Build::new()
        .file("resources/wrapper.h")
        .compile("inlinehook");
}
