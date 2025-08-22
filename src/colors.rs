#![allow(dead_code)] // Permite tener constantes no usadas sin generar advertencias.
//! Módulo para definir la paleta de colores de VesperOS.
//!
//! Puedes ajustar estos valores para que coincidan con los colores de tu README.

/// Tipo para representar un color en formato ARGB (Alpha, Red, Green, Blue).
/// Cada componente es un byte (0-255).
/// Por ejemplo, 0x00RRGGBB donde 0x00 es el canal alfa (totalmente opaco).
pub type Color = u32;

// Colores básicos
pub const BLACK: Color = 0x00000000;
pub const WHITE: Color = 0x00FFFFFF;
pub const RED: Color = 0x00FF0000;
pub const GREEN: Color = 0x0000FF00;
pub const BLUE: Color = 0x000000FF;
pub const TRANSPARENT: Color = 0xFF000000; // Usamos un valor sentinela para la transparencia

// Colores sugeridos para VesperOS (ajusta según tu README)
pub const BACKGROUND_COLOR: Color = 0x001A1A2E; // Un azul oscuro/morado
pub const ACCENT_COLOR: Color = 0x00E94560; // Un rojo rosado para acentos

// Paleta extendida de VesperOS
pub const DEEP_BLACK: Color = 0x000B0B0D;
pub const DARK_PURPLE: Color = 0x002C1A47;
pub const COSMIC_BLUE: Color = 0x001E2A78;
pub const BRIGHT_VIOLET: Color = 0x00A259FF;
pub const SMOKE_WHITE: Color = 0x00EAEAEA;
pub const NEON_GREEN: Color = 0x003DFFB4;

// Colores para texto
pub const TEXT_PRIMARY: Color = SMOKE_WHITE;
pub const TEXT_SECONDARY: Color = BRIGHT_VIOLET;
pub const TEXT_ACCENT: Color = NEON_GREEN;