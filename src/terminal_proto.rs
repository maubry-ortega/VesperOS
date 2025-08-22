//! Módulo que implementa el parser para el protocolo VIMG.
//!
//! Este parser está diseñado como una máquina de estados para poder procesar
//! datos de entrada que lleguen en fragmentos (chunks).

use crate::proto::{VimgHeader, MAGIC, cmd};
use core::mem::{self, size_of};

/// Estados posibles del parser.
pub enum ParseState {
    /// Esperando suficientes bytes para completar el header.
    WantHeader {
        buf: [u8; 32], // size_of::<VimgHeader>()
        filled: usize,
    },
    /// Ya tenemos un header válido, esperando los bytes de los datos de la imagen.
    WantData {
        header: VimgHeader,
        data_left: usize,
    },
}

/// El parser para el protocolo VIMG.
pub struct VimgParser {
    state: ParseState,
}

impl VimgParser {
    /// Crea un nuevo parser en su estado inicial.
    pub const fn new() -> Self {
        Self {
            state: ParseState::WantHeader { buf: [0; 32], filled: 0 },
        }
    }

    /// Reinicia el parser a su estado inicial.
    pub fn reset(&mut self) {
        self.state = ParseState::WantHeader { buf: [0; 32], filled: 0 };
    }

    /// Alimenta al parser con un fragmento de bytes de entrada.
    ///
    /// `draw_cb` es un callback que se ejecutará una única vez cuando se haya
    /// recibido y validado un paquete completo (header + datos).
    pub fn feed<F: FnOnce(&VimgHeader, &[u8]) -> Result<(), ()>>(
        &mut self,
        mut input: &[u8],
        draw_cb: F,
    ) -> Result<(), ()> {
        loop {
            match &mut self.state {
                ParseState::WantHeader { buf, filled } => {
                    let need = size_of::<VimgHeader>() - *filled;
                    let take = need.min(input.len());

                    buf[*filled..*filled + take].copy_from_slice(&input[..take]);
                    *filled += take;
                    input = &input[take..];

                    if *filled < size_of::<VimgHeader>() {
                        return Ok(()); // Aún no tenemos el header completo, esperamos más datos.
                    }

                    let header: VimgHeader = unsafe { mem::transmute_copy(buf) };

                    if header.magic != MAGIC || header.version != 1 || header.cmd != cmd::PUT {
                        self.reset();
                        return Err(()); // Error de protocolo, reiniciamos.
                    }

                    self.state = ParseState::WantData { header, data_left: header.data_len as usize };
                }
                ParseState::WantData { header, data_left } => {
                    if input.len() < *data_left {
                        self.reset();
                        return Err(()); // Datos incompletos.
                    }

                    let data = &input[..*data_left];
                    let res = draw_cb(header, data);
                    self.reset();
                    return res;
                }
            }
        }
    }
}