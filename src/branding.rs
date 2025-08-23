//! Módulo para definir la identidad visual y los assets de VesperOS.
//!
//! Este módulo centraliza los recursos gráficos, como el logo, para que sean
//! fácilmente accesibles desde otras partes del kernel.

use crate::colors::Color;

// Incluye el código del logo generado automáticamente por el script `build.rs`.
// Este `include!` expande el contenido del archivo `generated_logo.rs` (que se
// encuentra en el directorio de compilación `OUT_DIR`) directamente aquí.
// Esto nos permite cambiar el archivo PNG del logo y que el kernel se actualice
// solo al recompilar, sin necesidad de modificar el código fuente manually.
include!(concat!(env!("OUT_DIR"), "/generated_logo.rs"));

/// Representa una imagen cruda incrustada en el binario del kernel.
///
/// Contiene las dimensiones de la imagen y una referencia a un slice estático
/// de píxeles. El formato de color de los píxeles es `u32` (0x00RRGGBB).
pub struct RawImage {
    /// Ancho de la imagen en píxeles.
    pub width: usize,
    /// Alto de la imagen en píxeles.
    pub height: usize,
    /// Slice con los datos de los píxeles de la imagen, fila por fila.
    pub data: &'static [Color],
}

/// Logo oficial de VesperOS.
///
/// Los datos de esta imagen se cargan desde el archivo generado por `build.rs`,
/// a partir de la imagen PNG original.
pub const VESPER_LOGO_IMAGE: RawImage = RawImage {
    width: LOGO_WIDTH,
    height: LOGO_HEIGHT,
    data: &LOGO_PIXEL_DATA,
};