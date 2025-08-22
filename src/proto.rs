//! Módulo que define el protocolo binario "VIMG" para renderizar imágenes.

#![allow(dead_code)]

/// El header del paquete VIMG.
/// Se define como `repr(C, packed)` para asegurar un layout de memoria predecible
/// y sin padding, lo que permite reinterpretarlo desde un buffer de bytes.
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct VimgHeader {
    pub magic: [u8; 4],   // Identificador único: b"VIMG"
    pub version: u8,      // Versión del protocolo, ej: 1
    pub cmd: u8,          // Comando a ejecutar, ej: 0x01 (PUT)
    pub flags: u16,       // Bitfield para opciones (ej: transparencia)
    pub x: u32,           // Coordenada X (little-endian)
    pub y: u32,           // Coordenada Y (little-endian)
    pub w: u32,           // Ancho de la imagen (little-endian)
    pub h: u32,           // Alto de la imagen (little-endian)
    pub pixfmt: u8,       // Formato de los píxeles (ej: RGB888, BMP)
    pub reserved: [u8;3], // Reservado para uso futuro, debe ser 0
    pub data_len: u32,    // Longitud de los datos de píxeles (little-endian)
    pub crc32: u32,       // Checksum de los datos (little-endian)
}

pub const MAGIC: [u8; 4] = *b"VIMG";

pub mod cmd {
    pub const PUT: u8 = 0x01; // Comando para dibujar una imagen
}

pub mod pixfmt {
    pub const RGB888: u8 = 0; // Formato raw RGB, 3 bytes por píxel
    pub const BMP: u8 = 1;    // Formato de imagen BMP
}