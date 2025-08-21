fn main() {
    println!("cargo:rerun-if-changed=src/boot/multiboot2_header.S");
    cc::Build::new()
        .file("src/boot/multiboot2_header.S")
        .compile("multiboot2_header");
}
