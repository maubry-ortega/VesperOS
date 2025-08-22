//! Módulo para definir la identidad visual y assets de VesperOS.

use crate::colors::Color;

// Incluye el logo generado automáticamente por el script `build.rs`.
// Esto nos permite cambiar el PNG y que el kernel se actualice solo al recompilar.
include!(concat!(env!("OUT_DIR"), "/generated_logo.rs"));

/// Representa una imagen cruda incrustada en el binario.
pub struct RawImage {
    pub width: usize,
    pub height: usize,
    pub data: &'static [Color],
}

/// Logo de VesperOS. Los datos se cargan desde el archivo generado por `build.rs`.
pub const VESPER_LOGO_IMAGE: RawImage = RawImage {
    width: LOGO_WIDTH,
    height: LOGO_HEIGHT,
    data: &LOGO_PIXEL_DATA,
};