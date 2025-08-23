//! La shell de VesperOS.
//!
//! Este módulo proporciona una interfaz de línea de comandos (CLI) interactiva.
//! Gestiona la entrada del usuario, el parseo de comandos y su ejecución.

pub mod command;

use crate::app;
use crate::colors;
use crate::vga::FramebufferWriter;
use core::fmt::Write;
use self::command::{parse, Command};
use crate::arch::target::keyboard;
use heapless::String;

const PROMPT: &str = "vesper> ";
const MAX_BUFFER_SIZE: usize = 256;

/// Representa el estado de la shell.
pub struct Shell {
    /// Búfer que almacena la línea de comando actual que el usuario está escribiendo.
    buffer: String<MAX_BUFFER_SIZE>,
    // Historial de comandos para uso futuro (ej. flechas arriba/abajo).
    // history: heapless::Vec<String<MAX_BUFFER_SIZE>, 16>,
}

impl Shell {
    /// Crea una nueva instancia de la shell.
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            // history: heapless::Vec::new(),
        }
    }

    /// Dibuja el prompt de la shell en la posición actual del cursor.
    pub fn draw_prompt(&self, writer: &mut FramebufferWriter) {
        writer.set_color(colors::NEON_GREEN);
        // El resultado se ignora porque la escritura en el framebuffer no debería fallar.
        let _ = write!(writer, "{}", PROMPT);
        writer.set_color(colors::TEXT_PRIMARY);
        let _ = write!(writer, "{}", self.buffer);
    }

    /// Procesa un carácter de entrada recibido desde el teclado.
    ///
    /// # Arguments
    ///
    /// * `c`: El carácter Unicode recibido.
    /// * `writer`: Una referencia mutable al `FramebufferWriter` para dibujar en la pantalla.
    pub fn handle_input_char(&mut self, c: char, writer: &mut FramebufferWriter) {
        match c {
            '\n' => { // Tecla Enter
                let _ = writer.write_char('\n');
                self.run_command(writer);
                self.buffer.clear();
                self.draw_prompt(writer);
            }
            '\x08' => { // Backspace
                if self.buffer.pop().is_some() {
                    writer.backspace();
                }
            }
            // Caracteres imprimibles
            c if c.is_ascii_graphic() || c == ' ' => {
                if self.buffer.push(c).is_ok() {
                    let _ = writer.write_char(c);
                }
            }
            _ => {}
        }
    }

    /// Ejecuta el comando que está actualmente en el búfer.
    fn run_command(&mut self, writer: &mut FramebufferWriter) {
        let command = parse(self.buffer.as_str());
        match command {
            Command::Clear => {
                writer.clear(colors::BACKGROUND_COLOR);
            },
            Command::Echo(args) => {
                // El unwrap es seguro aquí porque la escritura en el framebuffer no debería fallar.
                writeln!(writer, "{}", args).unwrap();
            },
            Command::VesperFetch | Command::Info => {
                // Ejecuta la aplicación VesperFetch.
                app::vesperfetch_app::run(writer);

                // Pausa hasta que el usuario presione una tecla.
                writer.set_cursor_position(20, writer.height() - 40);
                writer.set_color(colors::TEXT_SECONDARY);
                let _ = write!(writer, "[Presiona cualquier tecla para continuar]");

                // Espera a que se presione y suelte una tecla.
                while keyboard::poll_key().is_some() {} // Drena eventos viejos si los hay.
                while keyboard::poll_key().is_none() {
                    crate::arch::wait_for_interrupt();
                }

                // Limpia la pantalla y vuelve a la shell.
                writer.clear(colors::BACKGROUND_COLOR);
            },
            Command::Help => {
                writeln!(writer, "Comandos de VesperOS:").unwrap();
                writeln!(writer, "  help         - Muestra esta ayuda.").unwrap();
                writeln!(writer, "  clear        - Limpia la pantalla.").unwrap();
                writeln!(writer, "  echo [msg]   - Imprime un mensaje.").unwrap();
                writeln!(writer, "  info         - Muestra la información del sistema.").unwrap();
            },
            Command::Unknown(cmd) => {
                writeln!(writer, "Comando no encontrado: {}", cmd).unwrap();
            },
            Command::None => {}
        }
    }
}