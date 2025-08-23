//! Módulo para el driver del teclado estándar PS/2.

use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    /// Instancia global del driver de teclado.
    /// Utiliza un Mutex para un acceso seguro desde el manejador de interrupciones
    /// y el bucle principal del kernel.
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        // El orden correcto de los argumentos es: ScancodeSet, Layout, HandleControl.
        Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));
}

lazy_static! {
    /// Una cola simple para los scancodes.
    /// El manejador de interrupciones escribe aquí, y el bucle principal lee.
    static ref SCANCODE_QUEUE: Mutex<heapless::Deque<u8, 8>> =
        Mutex::new(heapless::Deque::new());
}

/// Llamado por el manejador de interrupciones del teclado.
pub fn add_scancode(scancode: u8) {
    if SCANCODE_QUEUE.lock().push_back(scancode).is_err() {
        // La cola está llena: se pierde el carácter. Aceptable para un sistema simple.
    }
}

/// Sondea la cola en busca de un nuevo evento de teclado decodificado.
pub fn poll_key() -> Option<DecodedKey> {
    let mut scancode_queue = SCANCODE_QUEUE.lock();
    if let Some(scancode) = scancode_queue.pop_front() {
        let mut keyboard = KEYBOARD.lock();
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            return keyboard.process_keyevent(key_event);
        }
    }
    None
}
