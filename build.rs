//! Script de compilación para VesperOS.
//!
//! Este script se ejecuta automáticamente por Cargo antes de compilar el kernel.
//! Realiza dos tareas principales:
//! 1. Compila cualquier código ensamblador (`.S`) necesario para el kernel.
//! 2. Procesa la imagen del logo (PNG) y la convierte en un array de píxeles
//!    en formato Rust, que luego se incrusta directamente en el binario del kernel.

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    // --- Tarea 1: Compilación de código ensamblador ---

    // Indica a Cargo que vuelva a ejecutar este script si el archivo `halt.S` cambia.
    println!("cargo:rerun-if-changed=src/arch/x86_64/halt.S");
    println!("cargo:rerun-if-changed=src/arch/x86_64/interrupts.S");

    // Usa la crate `cc` para invocar al compilador C (que también maneja ensamblador)
    // y compilar `halt.S` en un objeto que se enlazará con el kernel.
    cc::Build::new()
        .file("src/arch/x86_64/halt.S")
        .file("src/arch/x86_64/interrupts.S")
        .compile("asm"); // El resultado se llamará `libasm.a`

    // --- Tarea 2: Procesamiento de la imagen del logo ---

    // Configuración de la imagen.
    const IMAGE_PATH: &str = "assets/vesper_pet_Nox_ansi.png";
    const TARGET_WIDTH: u32 = 64;
    const TARGET_HEIGHT: u32 = 64;
    const TRANSPARENT_COLOR: u32 = 0xFF000000;

    // 1. Definir la ruta del archivo de salida.
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_logo.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    // 2. Indicar a Cargo que vuelva a ejecutar si la imagen cambia.
    println!("cargo:rerun-if-changed={}", IMAGE_PATH);

    // 3. Abrir la imagen original usando la crate `image`.
    let img = image::open(IMAGE_PATH).expect("No se pudo abrir el archivo del logo.");

    // 4. Redimensionar la imagen al tamaño deseado (64x64).
    // `FilterType::Triangle` es un filtro de reescalado de buena calidad.
    let resized = img.resize_exact(TARGET_WIDTH, TARGET_HEIGHT, image::imageops::FilterType::Triangle);
    let rgba_image = resized.to_rgba8(); // Convertir a formato RGBA de 8 bits por canal.

    // 5. Escribir las constantes de tamaño y el inicio del array en el archivo de salida.
    writeln!(&mut f, "/// Ancho del logo en píxeles, generado por build.rs.").unwrap();
    writeln!(&mut f, "pub const LOGO_WIDTH: usize = {};", TARGET_WIDTH).unwrap();
    writeln!(&mut f, "/// Alto del logo en píxeles, generado por build.rs.").unwrap();
    writeln!(&mut f, "pub const LOGO_HEIGHT: usize = {};", TARGET_HEIGHT).unwrap();
    writeln!(&mut f, "/// Datos de píxeles del logo (formato 0x00RRGGBB), generado por build.rs.").unwrap();
    writeln!(&mut f, "pub static LOGO_PIXEL_DATA: [u32; {}] = [", TARGET_WIDTH * TARGET_HEIGHT).unwrap();

    // 6. Iterar sobre los píxeles de la imagen redimensionada y convertirlos a u32.
    for pixel in rgba_image.pixels() {
        let r = pixel[0] as u32;
        let g = pixel[1] as u32;
        let b = pixel[2] as u32;
        let a = pixel[3] as u32;

        // Si el píxel es mayormente transparente (alfa < 128), usamos nuestro color sentinela.
        // De lo contrario, combinamos los canales R, G, B en un u32 (0x00RRGGBB).
        let color = if a < 128 { TRANSPARENT_COLOR } else { (r << 16) | (g << 8) | b };
        writeln!(&mut f, "    0x{:08X},", color).unwrap();
    }

    writeln!(&mut f, "];").unwrap();
}