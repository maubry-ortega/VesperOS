#![allow(dead_code)] // Permite tener constantes no usadas sin generar advertencias.
//! Módulo para definir la paleta de colores de VesperOS.
//!
//! Los colores se definen como constantes `u32` en formato 0x00RRGGBB.

/// Tipo para representar un color en formato 0x00RRGGBB (Alpha, Red, Green, Blue).
/// El canal alfa no se usa actualmente para la mezcla, pero se reserva.
/// Un valor de 0x00 en el canal alfa indica opacidad total.
pub type Color = u32;

// --- Colores Básicos ---

/// Negro puro.
pub const BLACK: Color = 0x00000000;
/// Blanco puro.
pub const WHITE: Color = 0x00FFFFFF;
/// Rojo puro.
pub const RED: Color = 0x00FF0000;
/// Verde puro.
pub const GREEN: Color = 0x0000FF00;
/// Azul puro.
pub const BLUE: Color = 0x000000FF;
/// Color sentinela para la transparencia.
///
/// Se utiliza para indicar que un píxel no debe ser dibujado, permitiendo
/// que el fondo existente se mantenga visible. El valor `0xFF` en el canal alfa
/// se usa para distinguirlo de colores opacos.
pub const TRANSPARENT: Color = 0xFF000000;

// --- Paleta Principal de VesperOS ---

/// Color de fondo principal para la interfaz. (Negro Profundo)
pub const DEEP_BLACK: Color = 0x000B0B0D;
/// Color secundario oscuro. (Morado Oscuro)
pub const DARK_PURPLE: Color = 0x002C1A47;
/// Color para elementos de borde o fondos secundarios. (Azul Cósmico)
pub const COSMIC_BLUE: Color = 0x001E2A78;
/// Color de acento principal para etiquetas y elementos interactivos. (Violeta Brillante)
pub const BRIGHT_VIOLET: Color = 0x00A259FF;
/// Color principal para el texto. (Blanco Humo)
pub const SMOKE_WHITE: Color = 0x00EAEAEA;
/// Color de acento secundario para destacar información. (Verde Neón)
pub const NEON_GREEN: Color = 0x003DFFB4;

// --- Colores Semánticos para UI ---

/// Color de fondo por defecto para la mayoría de las pantallas.
pub const BACKGROUND_COLOR: Color = DEEP_BLACK;
/// Color de acento por defecto.
pub const ACCENT_COLOR: Color = BRIGHT_VIOLET;

/// Color primario para el texto (el más común).
pub const TEXT_PRIMARY: Color = SMOKE_WHITE;
/// Color secundario para el texto (menos importante, como etiquetas).
pub const TEXT_SECONDARY: Color = BRIGHT_VIOLET;
/// Color de acento para el texto (para resaltar).
pub const TEXT_ACCENT: Color = NEON_GREEN;