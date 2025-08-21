fn main() {
    // Compila nuestro archivo de ensamblador y lo enlaza al kernel.
    cc::Build::new()
        .file("src/arch/x86_64/halt.S")
        .compile("asm");
}