//! Módulo que define el protocolo binario "VIMG" para renderizar imágenes.
//!
//! VIMG es un protocolo simple diseñado para enviar imágenes a través de un
//! puerto serie o cualquier otro stream de bytes para ser renderizadas por el kernel.

#![allow(dead_code)]

/// El header de un paquete VIMG.
///
/// Se define como `repr(C, packed)` para asegurar un layout de memoria predecible
/// y sin padding. Esto es crucial porque permite reinterpretar de forma segura
/// (`transmute`) un buffer de bytes directamente a esta estructura.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct VimgHeader {
    /// Identificador único del protocolo: `b"VIMG"`.
    pub magic: [u8; 4],
    /// Versión del protocolo, ej: 1.
    pub version: u8,
    /// Comando a ejecutar. Ver el módulo `cmd`.
    pub cmd: u8,
    /// Bitfield para opciones futuras (ej: compresión, transparencia).
    pub flags: u16,
    /// Coordenada X (en formato little-endian) donde dibujar la imagen.
    pub x: u32,
    /// Coordenada Y (en formato little-endian) donde dibujar la imagen.
    pub y: u32,
    /// Ancho de la imagen en píxeles (little-endian).
    pub w: u32,
    /// Alto de la imagen en píxeles (little-endian).
    pub h: u32,
    /// Formato de los píxeles. Ver el módulo `pixfmt`.
    pub pixfmt: u8,
    /// Reservado para uso futuro, debe ser `[0, 0, 0]`.
    pub reserved: [u8;3],
    /// Longitud en bytes de los datos de píxeles que siguen a este header (little-endian).
    pub data_len: u32,
    /// Checksum CRC32 de los datos de píxeles para verificar la integridad (little-endian).
    pub crc32: u32,
}

/// El identificador mágico que debe estar al inicio de cada paquete VIMG.
pub const MAGIC: [u8; 4] = *b"VIMG";

/// Define los comandos disponibles en el protocolo VIMG.
pub mod cmd {
    /// Comando para dibujar una imagen en las coordenadas especificadas.
    pub const PUT: u8 = 0x01;
}

/// Define los formatos de píxeles soportados por el protocolo VIMG.
pub mod pixfmt {
    /// Formato raw RGB, 3 bytes por píxel (R, G, B).
    pub const RGB888: u8 = 0;
    /// Formato de imagen BMP, los datos son un archivo .bmp completo.
    pub const BMP: u8 = 1;
}