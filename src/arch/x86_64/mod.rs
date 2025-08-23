//! Implementación de la arquitectura x86_64.

pub mod interrupts;
pub mod keyboard; // El driver del teclado es específico de la arquitectura de PC.
mod cpu;
mod idt;

/// Inicializa la IDT y el controlador de interrupciones (PIC).
pub fn init() {
    interrupts::init();
}

/// Pone la CPU en estado de bajo consumo (HLT) hasta la próxima interrupción.
pub fn wait_for_interrupt() {
    cpu::hlt();
}