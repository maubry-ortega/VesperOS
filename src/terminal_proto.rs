//! Módulo que implementa un parser para el protocolo VIMG.
//!
//! Este parser está diseñado como una máquina de estados para poder procesar
//! datos de entrada que lleguen en fragmentos (chunks), por ejemplo, desde
//! un puerto serie. Mantiene su estado entre llamadas a `feed`.

use crate::proto::{VimgHeader, MAGIC, cmd};
use core::mem::{self, size_of};

/// Representa los estados posibles del parser VIMG.
pub enum ParseState {
    /// Esperando recibir suficientes bytes para completar el `VimgHeader`.
    ///
    /// Almacena los bytes recibidos hasta ahora en un buffer.
    WantHeader {
        /// Buffer para ensamblar el header. Su tamaño es `size_of::<VimgHeader>()`.
        buf: [u8; 32],
        /// Número de bytes ya escritos en `buf`.
        filled: usize,
    },
    /// Se ha recibido y validado un header. Ahora se esperan los datos de la imagen.
    WantData {
        /// El header validado.
        header: VimgHeader,
        /// El número de bytes de datos de la imagen que aún faltan por recibir.
        data_left: usize,
    },
}

/// El parser para el protocolo VIMG.
///
/// Mantiene el estado de la decodificación de un stream de bytes VIMG.
pub struct VimgParser {
    state: ParseState,
}

impl VimgParser {
    /// Crea un nuevo parser en su estado inicial (`WantHeader`).
    pub const fn new() -> Self {
        Self {
            state: ParseState::WantHeader { buf: [0; 32], filled: 0 },
        }
    }

    /// Reinicia el parser a su estado inicial.
    ///
    /// Útil para recuperarse de un error de protocolo sin crear una nueva instancia.
    pub fn reset(&mut self) {
        self.state = ParseState::WantHeader { buf: [0; 32], filled: 0 };
    }

    /// Alimenta al parser con un fragmento de bytes de entrada.
    ///
    /// Esta función procesa los bytes de `input` y avanza la máquina de estados.
    /// Cuando se recibe un paquete completo y válido (header + datos), se invoca
    /// el `draw_cb` con el header y los datos de la imagen.
    ///
    /// # Arguments
    ///
    /// * `input`: Un slice de bytes que representa el fragmento de datos recibido.
    /// * `draw_cb`: Un callback que se ejecutará una única vez cuando se haya
    ///   recibido y validado un paquete completo.
    ///
    /// # Returns
    ///
    /// * `Ok(())` si los datos se procesaron correctamente (aunque el paquete
    ///   pueda no estar completo todavía).
    /// * `Err(())` si se detecta un error de protocolo (ej: magic incorrecto,
    ///   datos incompletos). En caso de error, el parser se reinicia.
    pub fn feed<F: FnOnce(&VimgHeader, &[u8]) -> Result<(), ()>>(
        &mut self,
        mut input: &[u8],
        draw_cb: F,
    ) -> Result<(), ()> {
        // Bucle para procesar el input, ya que un chunk de input podría contener
        // un header y parte de los datos, requiriendo múltiples transiciones de estado.
        loop {
            match &mut self.state {
                ParseState::WantHeader { buf, filled } => {
                    let need = size_of::<VimgHeader>() - *filled;
                    let take = need.min(input.len());

                    buf[*filled..*filled + take].copy_from_slice(&input[..take]);
                    *filled += take;
                    input = &input[take..];

                    // Si aún no tenemos el header completo, salimos y esperamos más datos.
                    if *filled < size_of::<VimgHeader>() {
                        return Ok(());
                    }

                    // Una vez que el buffer está lleno, lo reinterpretamos como un VimgHeader.
                    // Es `unsafe` porque confiamos en que `repr(C, packed)` ha funcionado.
                    let header: VimgHeader = unsafe { mem::transmute_copy(buf) };

                    // Validamos el header. Si es inválido, reiniciamos y devolvemos error.
                    if header.magic != MAGIC || header.version != 1 || header.cmd != cmd::PUT {
                        self.reset();
                        return Err(());
                    }

                    // Transición de estado: ahora esperamos los datos.
                    self.state = ParseState::WantData { header, data_left: header.data_len as usize };
                }
                ParseState::WantData { header, data_left } => {
                    // En este protocolo simple, esperamos que los datos lleguen en un solo chunk
                    // después del header. Si no es así, es un error.
                    if input.len() < *data_left {
                        self.reset();
                        return Err(()); // Datos incompletos.
                    }

                    // Tenemos todos los datos. Los extraemos y llamamos al callback.
                    let data = &input[..*data_left];
                    let res = draw_cb(header, data);

                    // Después de procesar un paquete, reiniciamos para el siguiente.
                    self.reset();
                    return res;
                }
            }
        }
    }
}