// build.rs
fn main() {
    println!("cargo:rerun-if-changed=src/arch/x86_64/halt.S");
    
    // Compila nuestro archivo de ensamblador
    cc::Build::new()
        .file("src/arch/x86_64/halt.S")
        .compile("asm");
}