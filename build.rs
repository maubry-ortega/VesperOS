//! Script de compilación para convertir automáticamente el logo PNG a un array de Rust.

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    // --- Compilación de ensamblador ---
    println!("cargo:rerun-if-changed=src/arch/x86_64/halt.S");
    cc::Build::new()
        .file("src/arch/x86_64/halt.S")
        .compile("asm");

    // --- Configuración ---
    // La imagen PNG que queremos convertir.
    const IMAGE_PATH: &str = "assets/vesper_pet_Nox_ansi.png";
    // El color que tu kernel considera transparente.
    const TRANSPARENT_COLOR: u32 = 0xFF000000;

    // 1. Definir la ruta del archivo de salida en el directorio de compilación.
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_logo.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    // 2. Indicar a Cargo que vuelva a ejecutar este script si la imagen cambia.
    println!("cargo:rerun-if-changed={}", IMAGE_PATH);

    // 3. Decodificar la imagen PNG.
    let decoder = png::Decoder::new(File::open(IMAGE_PATH).expect("No se pudo abrir el archivo del logo. ¿Existe en 'assets/vesper_pet_Nox.png'?"));
    let mut reader = decoder.read_info().unwrap();

    // Validar que la imagen esté en un formato que esperamos (RGBA, 8 bits por canal).
    if reader.info().color_type != png::ColorType::Rgba {
        panic!("La imagen del logo debe estar en formato RGBA.");
    }
    if reader.info().bit_depth != png::BitDepth::Eight {
        panic!("La imagen del logo debe tener una profundidad de 8 bits por canal.");
    }

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];

    // 4. Escribir las constantes y el array de píxeles en el archivo de salida.
    writeln!(&mut f, "pub const LOGO_WIDTH: usize = {};", info.width).unwrap();
    writeln!(&mut f, "pub const LOGO_HEIGHT: usize = {};", info.height).unwrap();
    writeln!(&mut f, "pub static LOGO_PIXEL_DATA: [u32; {}] = [", info.width * info.height).unwrap();

    // 5. Iterar sobre los píxeles y convertirlos al formato 0x00RRGGBB.
    for pixel in bytes.chunks_exact(4) { // chunks_exact(4) para [R, G, B, A]
        let r = pixel[0] as u32;
        let g = pixel[1] as u32;
        let b = pixel[2] as u32;
        let a = pixel[3] as u32;

        // Si el píxel es mayormente transparente, usamos nuestro color sentinela.
        let color = if a < 128 { TRANSPARENT_COLOR } else { (r << 16) | (g << 8) | b };
        writeln!(&mut f, "    0x{:08X},", color).unwrap();
    }

    writeln!(&mut f, "];").unwrap();
}