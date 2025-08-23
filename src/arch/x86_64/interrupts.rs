//! Módulo para manejar las interrupciones de la CPU en x86_64.

use lazy_static::lazy_static;
use super::{cpu, idt::InterruptDescriptorTable, keyboard};
use pic8259::ChainedPics;
use spin;

/// El offset para el controlador de interrupciones programable (PIC).
/// Las interrupciones del PIC empezarán en el vector 32 para no solaparse
/// con las excepciones de la CPU.
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

/// Instancia estática y segura del controlador de interrupciones.
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

/*
 * Declaraciones `extern` para que Rust conozca los símbolos de nuestras
 * rutinas de ensamblador.
 *
 * # Safety
 * El bloque es `unsafe` porque el compilador no puede verificar que estas funciones existan.
 */
unsafe extern "C" {
    fn timer_interrupt_stub();
    fn keyboard_interrupt_stub();
}

lazy_static! {
    /// La Tabla de Descriptores de Interrupciones (IDT) principal del kernel.
    ///
    /// La IDT es una estructura de datos usada por la CPU para saber qué código
    /// ejecutar cuando ocurre una interrupción.
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Asigna la dirección de la rutina de ensamblador a cada interrupción.
        // Esto es 100% compatible con Rust `stable`.
        idt[InterruptIndex::Timer.as_usize()].set_handler(timer_interrupt_stub as u64);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler(keyboard_interrupt_stub as u64);

        idt
    };
}

/// Inicializa la IDT y el controlador de interrupciones.
pub fn init() {
    IDT.load();
    unsafe { PICS.lock().initialize() };
    // Habilita las interrupciones globalmente.
    unsafe { cpu::enable_interrupts() };
}

/// Enumera los índices de las interrupciones de hardware que manejamos.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 { self as u8 }
    fn as_usize(self) -> usize { usize::from(self.as_u8()) }
}

/// Manejador en Rust para la interrupción del temporizador (timer).
/// Esta función es llamada desde el stub de ensamblador `timer_interrupt_stub`.
#[unsafe(no_mangle)] // Requerido por la edición 2024 para atributos `extern`.
pub extern "C" fn rust_timer_interrupt_handler() {
    // Aquí se podría incrementar un contador de 'ticks' del sistema.
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

/// Manejador en Rust para la interrupción del teclado.
/// Esta función es llamada desde el stub de ensamblador `keyboard_interrupt_stub`.
#[unsafe(no_mangle)] // Requerido por la edición 2024 para atributos `extern`.
pub extern "C" fn rust_keyboard_interrupt_handler() {
    // Lee el "scancode" del puerto de datos del teclado.
    let scancode: u8 = unsafe { cpu::inb(0x60) };
    keyboard::add_scancode(scancode);

    // Notifica al PIC que la interrupción ha sido manejada.
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}